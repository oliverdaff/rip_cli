# RIP CLI

A simple Rust cli program that uses the RIP IOC to match 
[indicators of compromise](https://taosecurity.blogspot.com/2018/11/the-origin-of-term-indicators-of.html "origin of term indicator") (IOC) found in text data.

## Motivation
Text content often contains indicators of compromise.  Sources include Twitter, web pages and pdf reports.  Often IOC extraction is done in slower languages, using Rust to provide this functionality enables large volumes of text to be processed quickly and with low overhead.

## Usage

```
TODO: Add example bash cmd
```

## Installation
While this library is in initial state of development installation is done using cargo.

```
git checkout https://github.com/oliverdaff/rip_cli
cargo test 
cargo install
```

## License
MIT © Oliver Daff
