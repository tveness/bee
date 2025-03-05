use anyhow::{bail, Result};
use colored::Colorize;
use itertools::Itertools;
use miniz_oxide::inflate::decompress_to_vec;
use postcard::from_bytes;
use serde::Deserialize;
use std::collections::HashMap;
use trie_rs::{
    inc_search::{IncSearch, Position},
    Trie,
};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WordMap(pub HashMap<String, Vec<String>>);

pub fn load_trie() -> Result<Trie<u8>> {
    let trie_bytes_compressed = include_bytes!("../sowpods_trie.postcard.miniz");
    let trie_bytes = decompress_to_vec(trie_bytes_compressed).unwrap();
    let trie = from_bytes(&trie_bytes).unwrap();

    Ok(trie)
}

pub fn print_answers(answers: &[Answer]) {
    for Answer { length, words } in answers {
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

pub fn get_answers(middle: char, others: &[char]) -> Result<Vec<Answer>> {
    let mut all_chars = others.to_vec();
    all_chars.push(middle);
    all_chars.sort();
    all_chars.dedup();
    let all_chars = all_chars;

    let pangram: Vec<char> = all_chars.clone();

    if others.is_empty() {
        bail!("Too short for legal words");
    }

    let first_char = all_chars.first().unwrap();

    let trie = load_trie()?;

    // We will cycle through each of the letters
    let mut search = trie.inc_search();
    let mut words = vec![];
    let mut answers: HashMap<usize, Vec<Word>> = HashMap::new();

    // Depth-first search on characters
    let pos = Position::from(search.clone());
    let mut visiting = vec![*first_char];
    let mut positions = vec![pos];

    let next_map: HashMap<char, char> = all_chars
        .iter()
        .zip(all_chars.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .collect();

    'outer: loop {
        if visiting.is_empty() {
            break;
        }

        // Try visiting what's up next
        let up_next = *visiting.last().unwrap();
        if search.peek(&(up_next as u8)).is_some() {
            // If it works, then progress the query
            search.query(&(up_next as u8));
            // Now try going for the first letter again
            visiting.push(*first_char);
            // Save position
            let current_pos = Position::from(search.clone());
            positions.push(current_pos);

            // Save exact matches
            let prefix: String = search.prefix();
            if prefix.contains(middle) && trie.exact_match(&prefix) {
                words.push(prefix);
            }
        } else {
            loop {
                if visiting.is_empty() {
                    break 'outer;
                }
                // If there is a successor to this letter, then we'll just replace up_next with
                // that
                let old_next = visiting.pop().unwrap();
                if let Some(new_next) = next_map.get(&old_next) {
                    visiting.push(*new_next);
                    break;
                } else {
                    // Otherwise, we'll have to walk back up the tree of positions
                    positions.pop();
                    if positions.is_empty() {
                        break 'outer;
                    }
                    let last_pos = positions.last().unwrap();
                    // Reset search
                    search = IncSearch::resume(&trie.0, *last_pos);
                }
            }
        }
    }

    // Collect words into proper answers
    for word in words {
        let pan = is_pangram(&word, &pangram);
        let l = word.len();
        let e = answers.entry(l).or_insert(vec![]);
        let w = Word {
            word: word.to_string(),
            pangram: pan,
        };
        e.push(w);
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
