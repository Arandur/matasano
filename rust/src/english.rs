use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct FreqCount {
    counts: HashMap<char, u64>,
    total: u64
}

impl FreqCount {
    pub fn from_file(mut file: File) -> io::Result<FreqCount> {
        let mut file_contents = String::new();

        try!(file.read_to_string(&mut file_contents));

        Ok(FreqCount::from_string(&*file_contents))
    }

    pub fn get_distribution(&self) -> HashMap<char, f64> {
        self.counts.iter()
            .map(|(&k, &v)| (k, (v as f64) / (self.total as f64)))
            .collect()
    }

    pub fn from_string(string: &str) -> FreqCount {
        let mut counts = HashMap::new();

        for c in string.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        FreqCount { counts: counts, total: string.len() as u64 }
    }
}

pub fn english_score(string: &str) -> io::Result<f64> {
    let known_freq = try!(File::open("en.txt")
                          .and_then(FreqCount::from_file));
    let this_freq = FreqCount::from_string(string);

    Ok(known_freq.counts.iter()
       .map(|(k, v)| (v, this_freq.counts.get(k).cloned().unwrap_or(0)))
       .map(|(&v1, v2)| (v1 as f64 / known_freq.total as f64, 
                         v2 as f64 / this_freq.total as f64))
       .map(|(v1, v2)| (v1 - v2).abs())
       .fold(0.0f64, |acc, x| acc + x))
}
