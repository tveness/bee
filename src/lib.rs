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
