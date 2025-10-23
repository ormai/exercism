use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Error;

#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,        // -n
    only_matching_files: bool, // -l
    case_insensitive: bool,    // -i
    inverted: bool,            // -v
    entire_line: bool,         // -x
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            only_matching_files: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            inverted: flags.contains(&"-v"),
            entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();
    for file_name in files {
        let mut buf_reader = BufReader::new(File::open(file_name)?);
        let mut file = String::new();
        buf_reader.read_to_string(&mut file)?;
        for (n, line) in file.lines().enumerate() {
            let line_lower = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            let pattern = if flags.case_insensitive {
                pattern.to_lowercase()
            } else {
                pattern.to_string()
            };

            let does_match = if flags.entire_line {
                line_lower == pattern
            } else {
                line_lower.contains(&pattern)
            };
            let does_match = if flags.inverted {
                !does_match
            } else {
                does_match
            };

            if does_match {
                if flags.only_matching_files {
                    if !matches.contains(&file_name.to_string()) {
                        matches.push(file_name.to_string());
                    }
                } else {
                    matches.push(format!(
                        "{}{}{line}",
                        if files.len() > 1 {
                            format!("{file_name}:")
                        } else {
                            "".to_string()
                        },
                        if flags.line_numbers {
                            format!("{}:", n + 1)
                        } else {
                            "".to_string()
                        }
                    ));
                }
            }
        }
    }
    Ok(matches)
}
