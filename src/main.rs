extern crate ripioc;
extern crate clap;


use ripioc::network_ioc::parse_network_iocs;
use std::io::{self, Read};
use std::fs;
use clap::{Arg, ArgGroup, App};



// [--input INPUT] default: stdin
// [--output OUTPUT] default: stdout
fn main() {

    let result = App::new("CLI for rip IOC")
    .version("0.1")
    .about("Parser for IOC from input text")
    .arg(
        Arg::with_name("stdin")
            .short('c')
            .long("stdin")
            .value_name("STDIN")
            .about("Read input from stdin")
            .takes_value(false)
    )
    .arg(
        Arg::with_name("input_file")
            .short('i')
            .long("input_file")
            .value_name("INPUT_FILE")
            .about("Read input from file")
            .takes_value(true)
    )
    .group(ArgGroup::with_name("input")
         .args(&["stdin", "input_file"])
         .required(true)).get_matches();

    let mut input = String::new();

    let read_input_result = if result.is_present("stdin"){
        io::stdin().read_to_string(&mut input)
    } else {
        let input_file = result.value_of("input_file").unwrap();
        fs::File::open(&input_file).and_then(|mut f| f.read_to_string(&mut input))
    };

    match read_input_result {
        Ok(_) => {
            let iocs = parse_network_iocs(&input);
            println!("{:?}", iocs);
        },
        Err(e) => println!("Error reading input {}", e),
    }
}



