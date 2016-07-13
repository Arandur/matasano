// Modules
use matasano::english;
use std::f64;
use rustc_serialize::base64;

// Structs
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;

// Traits
use matasano::crypt::Crypt;
use std::io::BufRead;
use rustc_serialize::hex::{FromHex, ToHex};
use rustc_serialize::base64::ToBase64;

struct DecryptionAttempt {
    guess: String,
    score: f64,
}

impl Default for DecryptionAttempt {
    fn default() -> DecryptionAttempt {
        DecryptionAttempt { guess: String::new(), score: f64::MAX }
    }
}

fn single_character_xor_try_decrypt(bytes: Vec<u8>) -> DecryptionAttempt {
    (0..256).map(|i| vec![i as u8])
        .map(|k| bytes.xor_decrypt_with(&k))
        .map(|v| String::from_utf8(v))
        .filter(|r| r.is_ok()).map(|r| r.unwrap())
        .fold(DecryptionAttempt::default(), |best_so_far, s| {
            let score = english::english_score(&s);

            match score.partial_cmp(&best_so_far.score) {
                Some(Ordering::Less) => DecryptionAttempt { 
                    guess: s, score: score 
                },
                _ => best_so_far
            }
        })
}

#[test]
fn c1() {
    let b1 = include_str!("resources/c1-hex").trim().from_hex()
        .unwrap()
        .as_slice()
        .to_base64(base64::Config { 
            char_set: base64::CharacterSet::Standard,
            newline: base64::Newline::LF,
            pad: true,
            line_length: None});
    let b2 = include_str!("resources/c1-base64");

    assert_eq!(b1, b2.trim());
}

#[test]
fn c2() {
    let b1  = include_str!("resources/c2-in").trim().from_hex().unwrap();
    let b2  = include_str!("resources/c2-key").trim().from_hex().unwrap();
    let out = include_str!("resources/c2-out").trim();

    assert_eq!(b1.xor_encrypt_with(&b2).as_slice().to_hex(), out);
}

#[test]
fn c3() {
    let crypttext = include_str!("resources/c3-in").from_hex().unwrap();

    let plaintext = single_character_xor_try_decrypt(crypttext).guess;

    assert_eq!(plaintext.trim(), include_str!("resources/c3-out").trim());
}

#[test]
fn c4() {
    let f = File::open("../resources/s1/c4-in").unwrap();
    let f = BufReader::new(f);

    let out = f.lines()
        .map(|rs| single_character_xor_try_decrypt(rs.unwrap().from_hex().unwrap()))
        .fold(DecryptionAttempt::default(), |best_so_far, x| {
            match x.score.partial_cmp(&best_so_far.score) {
                Some(Ordering::Less) => x,
                _ => best_so_far
            }
        }).guess;

    assert_eq!(out.trim(), include_str!("resources/c4-out").trim());
}
