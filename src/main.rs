extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    // We expect 3 arguments
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let source_file = File::open(args().nth(1).unwrap());
    let output_path = args().nth(2).unwrap();
    let mut input = BufReader::new(source_file.unwrap());
    let output = File::create(output_path).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Time: {:?}", start.elapsed());
}
