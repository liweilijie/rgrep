use clap::Parser;
mod parse;
use parse::*;
use regex::Regex;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(parse(try_from_str))]
    reg: String,
    input_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("reg = {}, input_file = {:?}", cli.reg, cli.input_file);

    let re = Regex::new(&cli.reg).unwrap();


    match cli.input_file {
        Some(f) => parse_file(re, f).unwrap(),
        None => (),
    };
}
