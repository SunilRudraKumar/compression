extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `Source` `Target`");
        return;
    }

    let source_path = args().nth(1).unwrap();
    let target_path = args().nth(2).unwrap();

    let input_file = File::open(&source_path).unwrap();
    let output_file = File::create(&target_path).unwrap();

    let mut input = BufReader::new(input_file);
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!(
        "Target len: {:?}",
        output.metadata().unwrap().len()
    );
    println!("Elapsed: {:?}", start.elapsed());
}
