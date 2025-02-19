use clap::Parser;
use std::io::{self, Read as _, Write as _};
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "base58")]
#[command(about = "Base58 encode or decode FILE, or standard input, to standard output.")]
struct Cli {
    /// Decode data
    #[arg(short = 'd', long = "decode")]
    decode: bool,

    /// Ignore garbage characters in decoded data
    #[arg(short = 'i', long = "ignore-garbage")]
    ignore_garbage: bool,

    /// When decoding, wrap lines after COLS characters (default 76).
    /// Use 0 to disable line wrapping
    #[arg(short = 'w', long = "wrap", default_value = "76")]
    wrap: usize,

    /// File to process (defaults to stdin if not provided)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    // Set up input source
    let mut input = Vec::new();
    match args.file {
        Some(path) => {
            let mut file = File::open(path)?;
            file.read_to_end(&mut input)?;
        }
        None => {
            io::stdin().read_to_end(&mut input)?;
        }
    }

    if args.decode {
        // Decode mode
        let input_str = String::from_utf8_lossy(&input);
        let filtered: String = if args.ignore_garbage {
            input_str
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect()
        } else {
            input_str.to_string()
        };

        match bs58::decode(filtered.trim()).into_vec() {
            Ok(decoded) => {
                if args.wrap > 0 {
                    // Write with line wrapping
                    for chunk in decoded.chunks(args.wrap) {
                        io::stdout().write_all(chunk)?;
                        println!();
                    }
                } else {
                    // Write without line wrapping
                    io::stdout().write_all(&decoded)?;
                }
            }
            Err(e) => {
                eprintln!("Error decoding base58: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Encode mode
        let encoded = bs58::encode(&input).into_string();
        
        if args.wrap > 0 {
            // Write with line wrapping
            for chunk in encoded.as_bytes().chunks(args.wrap) {
                io::stdout().write_all(chunk)?;
                println!();
            }
        } else {
            // Write without line wrapping
            print!("{encoded}");
        }
    }

    Ok(())
}
