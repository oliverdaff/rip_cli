# RIP CLI (rioc) [![CircleCI](https://circleci.com/gh/oliverdaff/rip_cli.svg?style=shield)](https://circleci.com/gh/oliverdaff/rip_cli)

A simple Rust cli program that uses the RIP IOC to match 
[indicators of compromise](https://taosecurity.blogspot.com/2018/11/the-origin-of-term-indicators-of.html "origin of term indicator") (IOC) found in text data.

## Motivation
Text content often contains indicators of compromise.  Sources include Twitter, web pages and pdf reports.  Often IOC extraction is done in slower languages, using Rust to provide this functionality enables large volumes of text to be processed quickly and with low overhead.

## Usage

```bash
rioc -h
CLI for rip IOC 0.1
Parser for IOC from input text

USAGE:
    rioc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
        --serde_json    Output using serde_json JSON format
    -V, --version       Prints version information

OPTIONS:
    -i, --input_file <INPUT_FILE>      Read input from file
    -o, --output_file <OUTPUT_FILE>    Write output to file
```

### Examples

```bash
echo "this has a baddomain.com" |   rioc # Read from stdin and write to sdout

echo "this has a baddomain.com" |   rioc -o /tmp/iocs.txt # Read from stdin and write to a file

rioc -o /tmp/iocs.txt -i /tmp/inputtext # Read from stdin and write to a file

```

### JSON Output

The default JSON output (rather than with the `--serde_json` flag) is shown below.

```json
{
            "network": {
                "url" :    [],
                "domain" : [], 
                "email" :  [],
                "ipv4" :   [],
                "ipv6" :   [],
                "hexurl" : [],
            },
            "hash":{
                "md5" :     [], 
                "sha1" :    [],
                "sha256" :  [],
                "sha512" :  [],
                "ssdeep" :  [],
            },
            "file" : {
                "doc" :   [],
                "exe" :   [],
                "flash" : [],
                "img" :   [],
                "mac" :   [],
                "web" :   [],
                "zip" :   [],
            },
            "vulns": {
                "cve" : []
            }
        }
```

## Installation
While this library is in initial state of development installation is done using cargo.

```bash
git checkout https://github.com/oliverdaff/rip_cli
cargo test 
cargo install --path .
```

## License
MIT © Oliver Daff
