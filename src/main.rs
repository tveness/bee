use anyhow::{bail, Result};
use bee::{get_answers, print_answers};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Must be of form bee MABCDE, where M is the middle letter");
    }
    if args[1] == "--help" {
        println!("{} {}
  A simple tool to solve the NYT spelling bee puzzle

USAGE: {} [CENTRAL LETTER][OTHER LETTERS]
  Will return all of the words of length 3 or more in the built-in dictionary which use M and any other combination of provided letters.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),);
        return Ok(());
    }

    let mut word = args[1].chars();
    let middle = word.next().unwrap();
    let others: Vec<char> = word.collect();
    println!("Central letter: {middle:?}");
    println!("Other letters: {others:?}");

    let mut letters: Vec<char> = others.clone();
    letters.push(middle);

    letters.sort();

    let answers = get_answers(middle, others)?;

    print_answers(&answers, &letters);
    Ok(())
}
