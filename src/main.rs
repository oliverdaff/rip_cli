extern crate ripioc;

use ripioc::network_ioc::parse_network_iocs;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            let found_iocs = parse_network_iocs(&input);
            println!("{:?}", found_iocs);
        }
        Err(err) => println!("Error {}", err),
    }
}
