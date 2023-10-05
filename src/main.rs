use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[1];

    // Check if the file exists, and if not, create it.
    if !fs::metadata(filename).is_ok() {
        println!("Creating a new file: {}", filename);
        fs::write(filename, "")?;
    }

    loop {
        println!("Enter your text (Ctrl+C to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)?;

        file.write_all(input.as_bytes())?;
    }
}
