use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

#[derive(Debug, Clone)]
pub struct Config {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    source: String,
    destination: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.mappings)
    }
}
impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source, self.destination)
    }
}

pub fn load_config(path: &str) -> Result<Config, Error> {
    File::open(path)
        .map(|f| BufReader::new(f))
        .map(|br| br.lines())
        .map(|ls| ls.map(|line| line.map(convert_line_to_mapping).unwrap()))
        .map(|m| m.collect::<Vec<Mapping>>())
        .map(|m| Config { mappings: m })
}

pub fn convert_line_to_mapping(l: String) -> Mapping {
    let splits = l.split(':').collect::<Vec<&str>>();
    match splits.len() {
        2 => Mapping {
            source: splits[0].to_string(),
            destination: splits[1].to_string(),
        },
        _ => panic!("Failed to parse line '{l}'.\nExpected format <source> : <destination>"),
    }
}
