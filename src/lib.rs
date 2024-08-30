use anyhow::{bail, Result};
use colored::Colorize;
use itertools::Itertools;
use miniz_oxide::inflate::decompress_to_vec;
use postcard::from_bytes;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WordMap(pub HashMap<String, Vec<String>>);

pub fn load_sorted_words() -> Result<WordMap> {
    let sorted_words_bytes_compressed = include_bytes!("../sowpods_sorted.postcard.miniz");
    let sorted_words_bytes = decompress_to_vec(sorted_words_bytes_compressed).unwrap();
    let sorted_words: WordMap = from_bytes(&sorted_words_bytes).unwrap();

    Ok(sorted_words)
}

pub fn print_answers(answers: &[Answer]) {
    for Answer { length, words } in answers {
        let mut words = words.clone();
        words.sort();
        words.dedup();
        print!("{:>2}: [ ", length);
        for word in words {
            if word.pangram {
                print!("{} ", word.word.red());
            } else {
                print!("{} ", word.word);
            }
        }
        println!("]");
    }
}

fn is_pangram(word: &str, sorted_letters: &[char]) -> bool {
    let test_letters: Vec<char> = word.chars().sorted().dedup().collect();
    sorted_letters == test_letters
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub length: usize,
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    word: String,
    pangram: bool,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.word.cmp(&other.word))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.word.cmp(&other.word)
    }
}

impl PartialOrd for Answer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.length.cmp(&other.length))
    }
}

impl Ord for Answer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length)
    }
}

pub fn get_answers(middle: char, others: Vec<char>) -> Result<Vec<Answer>> {
    let mut others = others;
    others.sort();
    others.dedup();

    let mut pangram = others.clone();
    pangram.push(middle);
    pangram.sort();
    pangram.dedup();
    let pangram = pangram;

    if others.is_empty() {
        bail!("Too short for legal words");
    }
    // Load initial sorted words
    let sorted_words: WordMap = load_sorted_words()?;

    // Generate all combinations
    let l = others.len();
    let mut answers: HashMap<usize, Vec<Word>> = HashMap::new();

    // Although minimum length is 4, the length of
    // unique letters may be just two e.g. mama
    for length in 1..=l {
        for comb in others.clone().into_iter().combinations(length) {
            let mut chosen_letters: Vec<char> = comb.into_iter().collect();
            chosen_letters.push(middle);
            chosen_letters.sort();
            let sorted_word: String = String::from_iter(chosen_letters);
            if let Some(words) = sorted_words.0.get(&sorted_word) {
                for word in words {
                    let l = word.len();
                    if l > 3 {
                        let entry = answers.entry(l).or_default();
                        let pangram = is_pangram(word, &pangram);
                        entry.push(Word {
                            word: word.clone(),
                            pangram,
                        });
                    }
                }
            }
        }
    }
    let mut answers: Vec<Answer> = answers
        .into_iter()
        .map(|(length, words)| Answer { length, words })
        .collect();
    answers.sort();

    Ok(answers)
}

pub fn print_analyse_answers(letters: &[char], answers: &[Answer]) {
    let number_of_words: usize = answers.iter().map(|x| x.words.len()).sum();

    let pangrams: usize = answers
        .iter()
        .map(|x| {
            x.words
                .iter()
                .map(|x| if x.pangram { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!();
    println!(
        "{}: {}, {}: {}",
        "WORDS".bold(),
        number_of_words,
        "PANGRAMS".bold(),
        pangrams
    );
    println!();

    // letter : length : number
    let mut letter_map: HashMap<char, HashMap<usize, usize>> = HashMap::new();
    let mut sum: HashMap<usize, usize> = HashMap::new();

    let mut letter_pairs: HashMap<(char, char), usize> = HashMap::new();

    for answer in answers {
        let length = answer.length;
        for word in &answer.words {
            // For each of the words of length `length`
            let first_char = word.word.chars().next().unwrap();

            // Increase the count e.g. increase lettermap[a][2] if this is a two-letter word
            let letter_entry = letter_map.entry(first_char).or_default();
            let length_entry = letter_entry.entry(length).or_insert(0);
            *length_entry += 1;

            let sum_entry = sum.entry(length).or_insert(0);
            *sum_entry += 1;

            let second_char = word.word.chars().nth(1).unwrap();
            let pair_entry = letter_pairs.entry((first_char, second_char)).or_insert(0);
            *pair_entry += 1;
        }
    }

    let sums: Vec<(usize, usize)> = sum.into_iter().sorted_by_key(|a| a.0).collect();

    print!("    ");
    for length in &sums {
        print!("{:<3}", length.0.to_string().bold());
    }
    let sigma = "Σ".to_owned().bold();
    let col = ":".to_owned().bold();
    println!("{sigma}");

    // Now print
    for l in letters {
        if let Some(hm) = letter_map.get(l) {
            let mut running_sum = 0;
            print!("{}{col}  ", l.to_uppercase().to_string().bold());
            for length in &sums {
                print!(
                    "{:<3}",
                    match hm.get(&length.0) {
                        Some(x) => {
                            running_sum += x;
                            x.to_string()
                        }
                        None => "-".to_string(),
                    }
                );
            }
            println!("{}", running_sum.to_string().bold());
        }
    }

    print!("{sigma}{col}  ");
    for (_, v) in &sums {
        print!("{:<3}", v.to_string().bold())
    }

    let sum_sum = sums.iter().fold(0, |acc, x| acc + x.1);

    println!("{sum_sum}");
    println!();

    // Now print pairs
    let flat_pairs = letter_pairs.iter().sorted_by_key(|((first, second), _)| {
        let mut s = first.to_string();
        s.push(*second);
        s
    });
    let mut old_first = letters[0];
    for ((first, second), count) in flat_pairs {
        if *first != old_first {
            println!();
        }
        old_first = *first;
        print!(
            "{}{}: {count:<3}",
            first.to_uppercase(),
            second.to_uppercase()
        );
    }
    println!();
}
