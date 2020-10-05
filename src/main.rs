use std::io::{self, Read};
use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "s", long = "sleep")]
    /// Loki's sleeping? Isn't he always
    sleep: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load loki's ASCII picture from the specified file.
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args(); // [2]
    let mut message = String::new();
    let eye = if options.sleep { "-" } else { "o" }; // [1]

    // Loki definitely can't bark.
    if message.to_lowercase() == "woof" {
        eprintln!("Loki's a cat. You dumbass!")
    }

    // If STDIN is provided then ignore the message
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    };

    // Read from ASCII file
    match &options.catfile {
        Some (path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;

            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        },
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red());
            println!("    =( I )=");
        }
    }
    Ok(())
}
