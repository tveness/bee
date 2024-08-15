use anyhow::{bail, Result};
use bee::{load_sorted_words, print_answers, WordMap};
use itertools::Itertools;
use std::{collections::HashMap, env};

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
    let sorted_words: WordMap = load_sorted_words()?;

    // Generate all combinations
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

    print_answers(&answers);
    Ok(())
}
