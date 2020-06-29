extern crate clap;
extern crate ripioc;

mod output;

use crate::output::json;
use clap::{App, Arg};
use ripioc::parse_all_iocs;
use std::fs;
use std::io::{self, Read};

// [--input INPUT] default: stdin
// [--output OUTPUT] default: stdout
fn main() -> io::Result<()> {
    let result = App::new("CLI for rip IOC")
        .version("0.1")
        .about("Parser for IOC from input text")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input_file")
                .value_name("INPUT_FILE")
                .help("Read input from file")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output_file")
                .value_name("OUTPUT_FILE")
                .help("Write output to file")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let mut input = String::new();

    let iocs = match result.value_of("input_file") {
        Some(input_file) => {
            fs::File::open(&input_file).and_then(|mut f| f.read_to_string(&mut input))
        }
        None => io::stdin().read_to_string(&mut input),
    }
    .map(|_| parse_all_iocs(&input))?;

    match result.value_of("output_file") {
        Some(out_file) => fs::write(out_file, json::output_json(&iocs)),
        None => {
            println!("{:?}", json::output_json(&iocs));
            Ok(())
        }
    }
}
