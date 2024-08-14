use std::{collections::HashMap, env};

use anyhow::{bail, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/*
#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct Word(String);
*/

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct WordMap(HashMap<String, Vec<String>>);

fn main() -> Result<()> {
    /* Process sowpods to sorted_sowpods */

    /*
    let sowpods = std::fs::read_to_string("sowpods.json").unwrap();

    let sowpods: Vec<Word> = serde_json::from_str(&sowpods).unwrap();

    let mut hm: HashMap<String, Vec<String>> = HashMap::new();

    for w in sowpods {
        let wl = w.0.to_lowercase();
        // Sort
        let mut chars: Vec<char> = wl.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        chars.dedup();
        let sorted_word: String = String::from_iter(chars);
        let e = hm.entry(sorted_word).or_insert(vec![]);
        e.push(wl);
    }

    let wm = WordMap(hm);
    let output_str = serde_json::to_string(&wm).unwrap();

    std::fs::write("sowpods_sorted.json", &output_str).unwrap();
    */

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
    //eprintln!("Loading words...");
    //let sorted_words_str = std::fs::read_to_string("sowpods_sorted.json").unwrap();
    let sorted_words_str = include_str!("../sowpods_sorted.json");
    let sorted_words: WordMap = serde_json::from_str(&sorted_words_str).unwrap();

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
