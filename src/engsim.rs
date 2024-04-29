use strsim::normalized_damerau_levenshtein;
use std::fs::File;
use std::iter::Map;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader, Lines};
use std::{env, fs};

fn find(directory: &Path, filename: &Path) -> PathBuf {
    let candidate = directory.join(filename);

    if let Ok(metadata) = fs::metadata(&candidate) {
        if metadata.is_file() {
            return candidate;
        }
    }

    find(directory.parent().unwrap(), filename)
}

fn read_lines(file: File) -> io::Lines<io::BufReader<File>> {
    io::BufReader::new(file).lines()
}

pub struct SimAnalyzer {
    words: Vec<String>
}

impl SimAnalyzer {
    pub fn new() -> Self {
        let path = find(&env::current_dir().unwrap(), Path::new("words.txt"));
        let file = File::open(&path).unwrap();
        let words = read_lines(file).map(|l| l.unwrap().replace("\n", "").to_lowercase());
        SimAnalyzer {
            words: words.collect()
        }
    }

    pub fn confidence(&self, string: String) -> f64 {
        self.words.iter().map(|w: &String| normalized_damerau_levenshtein(&string.replace("\n", " ").to_lowercase(), &w)).fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
}

