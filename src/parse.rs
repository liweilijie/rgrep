use std::fs::File;
use std::io::{Result, BufReader, BufRead };
use std::path::Path;
use regex::Regex;

pub fn parse_file<P: AsRef<Path>>(reg: Regex, p: P) -> Result<()> {
    let file = File::open(p)?;
    let mut num = 1;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if reg.is_match(&line) {
            println!("{}: {}", num, &line);
        }
        num += 1;
    }

    Ok(())
}