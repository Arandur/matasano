use std::collections::HashMap;

fn canonical_english_freq() -> HashMap<char, f64> {
    let mut freqs = HashMap::with_capacity(27);
    
    freqs.insert(' ', 1.846E-1);
    freqs.insert('a', 6.478E-2);
    freqs.insert('b', 1.248E-2);
    freqs.insert('c', 2.102E-2);
    freqs.insert('d', 3.474E-2);
    freqs.insert('e', 1.030E-1);
    freqs.insert('f', 1.933E-2);
    freqs.insert('g', 1.596E-2);
    freqs.insert('h', 5.098E-2);
    freqs.insert('i', 5.641E-2);
    freqs.insert('j', 1.176E-3);
    freqs.insert('k', 5.768E-3);
    freqs.insert('l', 3.290E-2);
    freqs.insert('m', 2.068E-2);
    freqs.insert('n', 5.641E-2);
    freqs.insert('o', 6.196E-2);
    freqs.insert('p', 1.446E-2);
    freqs.insert('q', 8.689E-4);
    freqs.insert('r', 4.893E-2);
    freqs.insert('s', 5.209E-2);
    freqs.insert('t', 7.477E-2);
    freqs.insert('u', 2.294E-2);
    freqs.insert('v', 7.979E-3);
    freqs.insert('w', 1.808E-2);
    freqs.insert('x', 1.320E-3);
    freqs.insert('y', 1.565E-2);
    freqs.insert('z', 5.171E-4);

    freqs
}

pub fn english_score(string: &str) -> f64 {
    let mut this_freq = HashMap::new();

    for c in string.to_lowercase().chars() {
        *this_freq.entry(c).or_insert(0) += 1;
    }

    let this_freq: HashMap<char, f64> = this_freq.into_iter()
        .map(|(k, v)| (k, v as f64 / string.len() as f64))
        .collect();

    let known_freq = canonical_english_freq();

    known_freq.iter()
       .map(|(k, v)| (v, this_freq.get(k).cloned().unwrap_or(0.0)))
       .map(|(v1, v2)| (v1 - v2).abs())
       .fold(0.0, |acc, x| acc + x)
}
