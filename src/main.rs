//! A cli interface to the ripioc library.
//! rip_ioc will read text from the command line
//! or from a file and output the IOCs identified
//! in the text as JSON.
//! 
//! ## Example Usage
//! ```bash
//! echo "this has a baddomain.com" |   ripioc_cli # Read from stdin and write to sdout
//! 
//! echo "this has a baddomain.com" |   ipioc_cli -o /tmp/iocs.txt # Read from stdin and write to a file
//! 
//! ripioc_cli -o /tmp/iocs.txt -i /tmp/inputtext # Read from stdin and write to a file
//! ```
extern crate clap;
extern crate ripioc;

mod output;

use crate::output::json;
use clap::{App, Arg};
use ripioc::parse_all_iocs;
use std::fs;
use std::io::{self, Read, Write};

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
        .arg(
            Arg::with_name("serde_json")
                .long("serde_json")
                .help("Output using serde_json JSON format")
                .takes_value(false)
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

    let output = if result.is_present("serde_json") {
        json::output_json(&iocs)
    } else {
        json::output_non_serde_json(&iocs)
    };

    match result.value_of("output_file") {
        Some(out_file) => fs::write(out_file, output),
        None => io::stdout().write_all(output.as_bytes()),
    }
}
