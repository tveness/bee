use anyhow::{bail, Result};
use colored::Colorize;
use itertools::Itertools;
use miniz_oxide::inflate::decompress_to_vec;
use postcard::from_bytes;
use std::collections::{BTreeMap, HashMap};
use trie_rs::{
    inc_search::{IncSearch, Position},
    Trie,
};

/// Convenience type which holds the answers mapping from the length of the words to a collection
/// of words of that length
type Answers = BTreeMap<usize, Vec<Word>>;

/// This loads the compressed Trie of words which we are searching for the character in
pub fn load_trie() -> Result<Trie<u8>> {
    let trie_bytes_compressed = include_bytes!("../sowpods_trie.postcard.miniz");
    let trie_bytes = decompress_to_vec(trie_bytes_compressed).unwrap();
    let trie = from_bytes(&trie_bytes).unwrap();

    Ok(trie)
}

pub fn print_answers(answers: &Answers) {
    for (length, words) in answers {
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

/// Determines whether a `word` uses all of the letters, and only the letters, in `sorted_letters`
fn is_pangram(word: &str, sorted_letters: &[char]) -> bool {
    let test_letters: Vec<char> = word.chars().sorted().dedup().collect();
    sorted_letters == test_letters
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Word {
    word: String,
    pangram: bool,
}

pub fn get_answers(middle: char, others: &[char]) -> Result<Answers> {
    let mut all_chars = others.to_vec();
    all_chars.push(middle);
    all_chars.sort();
    all_chars.dedup();
    let all_chars = all_chars;

    if others.is_empty() {
        bail!("Too short for legal words");
    }

    let first_char = all_chars.first().unwrap();

    let trie = load_trie()?;

    // We will cycle through each of the letters
    let mut search = trie.inc_search();
    let mut length_word_map = Answers::new();

    // Depth-first search on characters
    let pos = Position::from(search.clone());
    let mut visiting = vec![*first_char];
    let mut positions = vec![pos];

    // A map of the next letter in the sequence
    // e.g. for [a, b, c, d] -> { a: b, b: c, c: d}
    let next_map: HashMap<char, char> = all_chars
        .iter()
        .zip(all_chars.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .collect();

    // Depth-first search
    loop {
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
                let pan = is_pangram(&prefix, &all_chars);
                let l = prefix.len();
                let w = Word {
                    word: prefix.to_string(),
                    pangram: pan,
                };
                let e = length_word_map.entry(l).or_default();

                // Insert is sorted as we visit words in alphabetical order
                e.push(w);
            }
        } else {
            'inner: loop {
                // If there is a successor to this letter, then replace up_next with that
                let old_next = visiting.pop().unwrap();
                if let Some(new_next) = next_map.get(&old_next) {
                    visiting.push(*new_next);
                    break 'inner;
                } else {
                    // Otherwise, we'll have to backtrack using the saved positions
                    positions.pop();
                    // If there are no positions saved, we are at the end of the line
                    if positions.is_empty() {
                        return Ok(length_word_map);
                    }
                    let last_pos = positions.last().unwrap();
                    // Reset search to this position
                    search = IncSearch::resume(&trie.0, *last_pos);
                }
            }
        }
    }
}

pub fn print_analyse_answers(letters: &[char], answers: &Answers) {
    let number_of_words: usize = answers.iter().map(|x| x.1.len()).sum();

    let pangrams: usize = answers
        .iter()
        .map(|x| {
            x.1.iter()
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

    for (&length, words) in answers {
        for word in words {
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
