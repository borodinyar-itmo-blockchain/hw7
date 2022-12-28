use clap::Parser;
use std::io::BufReader;
use std::{fs::File, io::BufRead};
use sha3::Digest;

#[derive(Parser, Debug)]
#[command(version,
    author = "Borodin Yaroslav", 
    about = "The program that generates ticket numbers is deterministically associated with the student's full name and the parameter that changes the distribution.", 
    long_about = None
)]
struct Args {
    /// Path to file with students full names
    #[arg(short, long)]
    file: String,

    /// Number of tickets
    #[arg(short, long)]
    numbilets: u64,

    /// Parameter that changes the distribution
    #[arg(short, long)]
    parameter: u64,
}

fn generate_ticket_number(name: &String, parameter: u64, max : u64) -> u64 {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(name.as_bytes());
    hasher.update(parameter.to_le_bytes());
    let result = hasher.finalize();

    let mut ticket_number = 0;
    for i in 1..9 {
        ticket_number <<= 8;
        ticket_number += result[i] as u64;
    }
    ticket_number %= max;
    ticket_number + 1
}

fn read_file(file: &String) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut names = Vec::new();
    for line in reader.lines() {
        names.push(line?);
    }
    Ok(names)
}

fn main() {
    let args = Args::parse();

    match read_file(&args.file) {
        Ok(names) => {
            for name in names {
                println!("{}: {}", name, generate_ticket_number(&name, args.parameter, args.numbilets));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
