use anyhow::{bail, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct WordMap(HashMap<String, Vec<String>>);

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Must be of form bee MABCDE, where M is the middle letter");
    }

    let mut word = args[1].chars();
    let middle = word.next().unwrap();
    let mut others: Vec<char> = word.collect();
    others.sort();
    others.dedup();

    println!("Central letter: {middle}");
    println!("Other letters: {others:?}");
    if others.len() < 3 {
        eprintln!("Too short for legal words");
    }

    // Load initial sorted words
    //let sorted_words_bytes = include_bytes!("../sowpods_sorted.postcard");
    //let sorted_words: WordMap = postcard::from_bytes(sorted_words_bytes).unwrap();

    let sorted_words_bytes_compressed = include_bytes!("../sowpods_sorted.postcard.miniz");
    let sorted_words_bytes =
        miniz_oxide::inflate::decompress_to_vec(sorted_words_bytes_compressed).unwrap();
    let sorted_words: WordMap = postcard::from_bytes(&sorted_words_bytes).unwrap();

    // Generate all combinations: words must be at least 4 letters, so we have a total of
    // L = others.len()
    // Total = 2^L - (L 2) - (L 3) combinations
    //       = 2^L - L(L-1)/2 - L(L-1)(L-2)/6
    let l = others.len();
    let mut answers: HashMap<usize, Vec<String>> = HashMap::new();

    for length in 1..=l {
        for comb in others.clone().into_iter().combinations(length) {
            let mut chosen_letters: Vec<char> = comb.into_iter().collect();
            chosen_letters.push(middle);
            chosen_letters.sort();
            let sorted_word: String = String::from_iter(chosen_letters);
            //println!("Chosen letters: {sorted_word}");
            if let Some(words) = sorted_words.0.get(&sorted_word) {
                for word in words {
                    let l = word.len();
                    if l > 3 {
                        let entry = answers.entry(l).or_default();
                        entry.push(word.clone());
                    }
                }
            }
        }
    }
    let mut answers: Vec<(usize, Vec<String>)> = answers.into_iter().collect();
    answers.sort_by(|a, b| a.0.cmp(&b.0));

    for (l, a) in answers {
        let mut a = a.clone();
        a.sort();
        a.dedup();
        println!("{}: {:?}", l, a);
    }
    Ok(())
}
