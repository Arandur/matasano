use super::languages::Language;

use std::f32;
use std::u32;
use std::cmp;
use std::ops;

pub trait EncryptWith<T> {
    fn encrypt_with(plaintext: &[u8], key: T) -> Vec<u8>;

    fn decrypt_with(crypttext: &[u8], key: T) -> Vec<u8>;
}

pub trait TryDecrypt {
    fn try_decrypt(crypttext: &[u8], language: Language) -> (Vec<u8>, f32);
}

pub struct SingleByteXor;

impl EncryptWith<u8> for SingleByteXor {
    fn encrypt_with(plaintext: &[u8], key: u8) -> Vec<u8> {
        plaintext.iter().map(|b| b ^ key).collect()
    }

    fn decrypt_with(crypttext: &[u8], key: u8) -> Vec<u8> {
        Self::encrypt_with(crypttext, key)
    }
}

impl TryDecrypt for SingleByteXor {
    fn try_decrypt(crypttext: &[u8], language: Language) -> (Vec<u8>, f32) {
        (0u32..256u32).map(|k| SingleByteXor::decrypt_with(crypttext, k as u8))
            .map(|bs| (bs.clone(), language.compare(&bs)))
            .fold((Vec::new(), f32::MAX), |(best_bs, best_score), (bs, score)| {
                match score.partial_cmp(&best_score) {
                    Some(cmp::Ordering::Less) => (bs, score),
                    _ => (best_bs, best_score)
                }
            })
    }
}

pub struct MultiByteXor;

impl<'a> EncryptWith<&'a [u8]> for MultiByteXor {
    fn encrypt_with(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
        plaintext.iter().zip(key.iter().cycle()).map(|(b, k)| b ^ k).collect()
    }

    fn decrypt_with(crypttext: &[u8], key: &[u8]) -> Vec<u8> {
        Self::encrypt_with(crypttext, key)
    }
}

fn hamming_distance(a: &[u8], b: &[u8]) -> Option<u32> {
    if a.len() != b.len() {
        None
    } else {
        Some(a.iter().zip(b.iter())
             .map(|(x, y)| (x ^ y).count_ones())
             .fold(0, |acc, x| acc + x))
    }
}

// This is obviously duplicated code. Not sure how best to fix this yet. 
// Modify TryDecrypt to also return the key? We would need to add a new type
// parameter.
fn single_byte_xor_get_key(crypttext: &[u8], language: Language) -> u8 {
    (0u32..256u32).map(|k| (k as u8, SingleByteXor::decrypt_with(crypttext, k as u8)))
        .map(|(k, bs)| (k, bs.clone(), language.compare(&bs)))
        .fold((0u8, Vec::new(), f32::MAX), |(best_key, best_bs, best_score), (k, bs, score)| {
            match score.partial_cmp(&best_score) {
                Some(cmp::Ordering::Less) => (k, bs, score),
                _ => (best_key, best_bs, best_score)
            }
        }).0
}

impl TryDecrypt for MultiByteXor {
    fn try_decrypt(crypttext: &[u8], language: Language) -> (Vec<u8>, f32) {
        let keysizes = ops::Range { 
            start: 2, 
            end: cmp::max(40, crypttext.len() / 2) 
        };

        let block_scores = keysizes.clone()
            .map(|keysize| (ops::Range { start: 0, end: keysize },
                            ops::Range { start: keysize, end: 2 * keysize }))
            .map(|(r1, r2)| (&crypttext[r1], &crypttext[r2]))
            .map(|(b1, b2)| hamming_distance(b1, b2).unwrap());

        let keysize = keysizes.zip(block_scores)
            .fold((0, u32::MAX), |(best_ks, best_score), (ks, score)| {
                if score.lt(&best_score) { (ks, score) } else { (best_ks, best_score) }
            }).0;

        let blocks: Vec<&[u8]> = crypttext.chunks(keysize as usize).collect();
        let t_blocks: Vec<Vec<u8>> = ops::Range { start: 0, end: keysize }
            .map(|ks| blocks.iter().map(|b| b[ks as usize]).collect::<Vec<u8>>())
            .collect();

        let key: Vec<u8> = t_blocks.iter()
            .map(|b| single_byte_xor_get_key(&b, language.clone()))
            .collect();

        let plaintext = Self::decrypt_with(crypttext, &key);
        let score = language.compare(&plaintext);

        (plaintext, score)
    }
}
