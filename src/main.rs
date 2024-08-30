use anyhow::{bail, Result};
use argh::FromArgs;
use bee::{get_answers, print_analyse_answers, print_answers};

#[derive(FromArgs)]
/// A simple tool to solve the NYT spelling bee puzzle
struct Args {
    /// display stats instead of solution
    #[argh(switch, short = 's')]
    stats: bool,

    /// show version
    #[argh(switch)]
    version: bool,

    #[argh(
        positional,
        arg_name = "LETTERS",
        description = "must be of form [CENTRAL LETTER][OTHER LETTERS]
             This will then return all of the words of length 3 or more in the built-in dictionary which use the central letter and any other combination of provided letters."
    )]
    letters: Option<String>,
}

fn main() -> Result<()> {
    let Args {
        stats,
        letters,
        version,
    } = argh::from_env();

    // Version has higher
    if version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Unwrap the letters
    let Some(letters) = letters else {
        bail!("Must provide letters with which to solve!");
    };

    let middle = letters.chars().next().unwrap();
    let others: Vec<char> = letters.chars().skip(1).collect();
    println!("Central letter: {middle:?}");
    println!("Other letters: {others:?}");

    let answers = get_answers(middle, others.clone())?;

    if !stats {
        print_answers(&answers);
    } else {
        // Ensure sorted
        let mut letters = others.clone();
        letters.push(middle);
        letters.sort();
        letters.dedup();

        print_analyse_answers(&letters, &answers);
    }

    Ok(())
}
