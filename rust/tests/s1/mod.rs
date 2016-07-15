use matasano::crypt::*;
use matasano::languages::*;

// Modules
use rustc_serialize::base64;
use std::f32;

// Structs
use std::cmp::Ordering;
use std::io::BufReader;
use std::fs::File;

// Traits
use std::io::BufRead;
use rustc_serialize::hex::{ToHex, FromHex};
use rustc_serialize::base64::{ToBase64, FromBase64};

#[test]
fn c1() {
    let b1 = include_str!("resources/c1-hex").trim().from_hex()
        .unwrap()
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
    let out = MultiByteXor::encrypt_with(&b1, &b2).to_hex();

    assert_eq!(out, include_str!("resources/c2-out").trim());
}

#[test]
fn c3() {
    let crypttext = include_str!("resources/c3-in").from_hex().unwrap();

    let (plaintext, _score) = 
        SingleByteXor::try_decrypt(&crypttext, Language::EnglishUtf8);
    let plaintext = String::from_utf8(plaintext).unwrap();

    assert_eq!(plaintext.trim(), include_str!("resources/c3-out").trim());
}

#[test]
fn c4() {
    let f = File::open("../resources/s1/c4-in").unwrap();
    let f = BufReader::new(f);

    let (out, _score) = f.lines()
        .map(|rs| SingleByteXor::try_decrypt(
                &rs.unwrap().from_hex().unwrap(), 
                Language::EnglishUtf8))
        .fold((Vec::new(), f32::MAX), |(bs_best, score_best), (bs, score)| {
            match score.partial_cmp(&score_best) {
                Some(Ordering::Less) => (bs, score),
                _ => (bs_best, score_best)
            }
        });
    let out = String::from_utf8(out).unwrap();

    assert_eq!(out.trim(), include_str!("resources/c4-out").trim());
}

#[test]
fn c5() {
    let plaintext = include_str!("resources/c5-in").trim().as_bytes();
    let key = include_str!("resources/c5-key").trim().as_bytes();
    
    let out = MultiByteXor::encrypt_with(plaintext, key).to_hex();

    assert_eq!(out, include_str!("resources/c5-out").trim());
}

#[test]
fn c6() {
    let crypttext = include_str!("resources/c6-in").from_base64().unwrap();

    let (plaintext, _score) = 
        MultiByteXor::try_decrypt(&crypttext, Language::EnglishUtf8);
    let plaintext = String::from_utf8(plaintext).unwrap();

    assert_eq!(plaintext.trim(), include_str!("resources/c6-out").trim());
}
