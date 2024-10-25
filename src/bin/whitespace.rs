use clap::Parser;

use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut bytes: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut bytes).unwrap();

    let mut bin: String = String::new();
    let mut end_bytes: Vec<u8> = Vec::new();
    
    for byte in bytes {
        if byte == 32 {
            bin.push_str("0");
        } else if byte == 9 {
            bin.push_str("1");
        }

        if bin.len() == 8 {
            end_bytes.push(u8::from_str_radix(&bin, 2).unwrap()); 
            bin = String::new();
        }
    }

    for byte in &end_bytes {
        print!("{} ", byte);
    }
    println!("{}", bin);

    println!("{:X?}", end_bytes);
}
