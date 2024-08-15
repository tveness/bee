use anyhow::{bail, Result};
use bee::{get_answers, print_answers};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Must be of form bee MABCDE, where M is the middle letter");
    }

    let mut word = args[1].chars();
    let middle = word.next().unwrap();
    let others: Vec<char> = word.collect();
    println!("Central letter: {middle:?}");
    println!("Other letters: {others:?}");

    let answers = get_answers(middle, others)?;

    print_answers(&answers);
    Ok(())
}
