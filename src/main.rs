extern crate colored;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;
use terminal_size::{terminal_size, Width};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[structopt(short = "d", long = "dead")]
    ///Make the cat appear dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    ///Load the cat picture from specified file
    catfile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    ///Read the message from STDIN instead of the argument
    stdin: bool,
    #[structopt(short = "w", long = "width", default_value = "40")]
    ///Max length of one row of symbols
    width: String,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };

    let width_result = options.width.parse::<usize>();
    let mut width = match width_result {
        Ok(num) => num,
        Err(_) => 0,
    };
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
        if width > w as usize {
            width = w as usize - 5;
        }
    }
    if width > message.len() {
        width = message.len();
    }
    if width >= 1 {
        let str_count = (message.len() - 1) / width;
        println!(" {} ", "-".repeat(width + 2));
        match str_count {
            0 => {
                println!("< {:1$} >", message, width);
            }

            oth => {
                println!("/ {} \\", &message[..width]);
                (1..=oth - 1)
                    .for_each(|i| println!("| {} |", &message[i * width..(i + 1) * width]));
                println!("\\ {:1$} /", &message[width * oth..], width);
            }
        }
        println!(" {} ", "-".repeat(width + 2));
    } else {
        println!(" --- ");
        println!("< 0 >");
        println!(" --- ");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        }
        None => {
            let half = width / 2;
            println!("{:1$} \\", " ", half);
            println!("{:1$}  \\", " ", half);
            println!("{:1$}     /\\_/\\", " ", half);
            println!(
                "{:1$}    ( {eye} {eye} )",
                " ",
                half,
                eye = eye.red().bold()
            );
            println!("{:1$}    =( I )=", " ", half);
        }
    }
    Ok(())
}
