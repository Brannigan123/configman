use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

/// `Config` is a struct that contains a vector of `Mapping`s.
///
/// Properties:
///
/// * `mappings`: A vector of Mapping structs.
#[derive(Debug, Clone)]
pub struct Config {
    pub mappings: Vec<Mapping>,
}

/// `Mapping` is a struct that contains two strings, `source` and `destination`.
///
/// Properties:
///
/// * `source`: The source path of the file to be copied.
/// * `destination`: The destination path of the file.
#[derive(Debug, Clone)]
pub struct Mapping {
    pub source: String,
    pub destination: String,
}

/// It's implementing the `Display` trait for the `Config` struct.
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.mappings)
    }
}

/// It's implementing the `Display` trait for the `Mapping` struct.
impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source, self.destination)
    }
}

/// We open the file, wrap it in a buffered reader, get the lines, filter out the ones we don't care
/// about, convert the ones we do care about to mappings, and then collect them into a vector inside
/// a Config Struct
///
/// Arguments:
///
/// * `path`: &str - The path to the config file
///
/// Returns:
///
/// A Result<Config, Error>
pub fn load_config(path: &str) -> Result<Config, Error> {
    File::open(path)
        .map(|f| BufReader::new(f))
        .map(|br| br.lines())
        .map(|ls| {
            ls.map(|l| l.unwrap())
                .filter(considered_mapping)
                .map(convert_line_to_mapping)
        })
        .map(|m| m.collect::<Vec<Mapping>>())
        .map(|m| Config { mappings: m })
}

/// If the line is empty or starts with a hash, it's not a mapping
///
/// Arguments:
///
/// * `l`: &String - the line to be considered
///
/// Returns:
///
/// A boolean value.
pub fn considered_mapping(l: &String) -> bool {
    let tl = l.trim();
    !(tl.is_empty() || tl.starts_with('#'))
}

/// It takes a string, splits it on the colon, and returns a Mapping struct
/// If resulting splits are 2, 1st is considered the source and the other a
/// destination. All other cases are considered illegal.
///
/// Arguments:
///
/// * `l`: The line to convert
///
/// Returns:
///
/// A vector of Mapping structs
pub fn convert_line_to_mapping(l: String) -> Mapping {
    let splits = l.split(':').collect::<Vec<&str>>();
    match splits.len() {
        2 => Mapping {
            source: splits[0].trim().to_string(),
            destination: splits[1].trim().to_string(),
        },
        _ => panic!("Failed to parse line '{l}'.\nExpected format <source> : <destination>"),
    }
}

pub const SAMPLE_CONFIG_CONTENT: &str = "
# This is a sample config file
# The configuration file supports Unix shell style patterns when matching files

# This mapping tracks the files in .mplayer directory and
# maps them to mplayer directory the under current working directory
# the brackets are used to capture groups of matched path which can
# be referenced on definition path as (<group position>),
# e.g (1) for this case corresponds to the first matched part in ()
/home/nomen/.mplayer/(*) : mplayer/(1)

# example
# ~/.config/neofetch/config.conf -> config/neofetch/config.conf
~/.config/(**) : config/(1)

# example
# ~/.config/polybar/launch.sh -> sh/polybar/launch.sh
~/.config/(*)/(*.sh) : sh/(1)/(2)

# ~/.config/neofetch/images/arch.png -> 
~/.config/(*)/**/(*.png) : (1)/pngs/(2) -> neofetch/pngs/arch.png

";
