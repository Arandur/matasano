use std::io;
use std::io::Read;

use itertools::Itertools;

fn hexchar_to_nibble(c: char) -> Result<u8, char> {
    match c {
        '0' => Ok(0x0), '1' => Ok(0x1), '2' => Ok(0x2), '3' => Ok(0x3),
        '4' => Ok(0x4), '5' => Ok(0x5), '6' => Ok(0x6), '7' => Ok(0x7),
        '8' => Ok(0x8), '9' => Ok(0x9), 'a' => Ok(0xA), 'A' => Ok(0xA),
        'b' => Ok(0xB), 'B' => Ok(0xB), 'c' => Ok(0xC), 'C' => Ok(0xC),
        'd' => Ok(0xD), 'D' => Ok(0xD), 'e' => Ok(0xE), 'E' => Ok(0xE),
        'f' => Ok(0xF), 'F' => Ok(0xF),  _  => Err(c)
    }
}

fn base64char_to_sextet(c: char) -> Result<u8, char> {
    match c {
        'A' => Ok(0o00), 'B' => Ok(0o01), 'C' => Ok(0o02), 'D' => Ok(0o03),
        'E' => Ok(0o04), 'F' => Ok(0o05), 'G' => Ok(0o06), 'H' => Ok(0o07),
        'I' => Ok(0o10), 'J' => Ok(0o11), 'K' => Ok(0o12), 'L' => Ok(0o13),
        'M' => Ok(0o14), 'N' => Ok(0o15), 'O' => Ok(0o16), 'P' => Ok(0o17),
        'Q' => Ok(0o20), 'R' => Ok(0o21), 'S' => Ok(0o22), 'T' => Ok(0o23),
        'U' => Ok(0o24), 'V' => Ok(0o25), 'W' => Ok(0o26), 'X' => Ok(0o27),
        'Y' => Ok(0o30), 'Z' => Ok(0o31), 'a' => Ok(0o32), 'b' => Ok(0o33),
        'c' => Ok(0o34), 'd' => Ok(0o35), 'e' => Ok(0o36), 'f' => Ok(0o37),
        'g' => Ok(0o40), 'h' => Ok(0o41), 'i' => Ok(0o42), 'j' => Ok(0o43),
        'k' => Ok(0o44), 'l' => Ok(0o45), 'm' => Ok(0o46), 'n' => Ok(0o47),
        'o' => Ok(0o50), 'p' => Ok(0o51), 'q' => Ok(0o52), 'r' => Ok(0o53),
        's' => Ok(0o54), 't' => Ok(0o55), 'u' => Ok(0o56), 'v' => Ok(0o57),
        'w' => Ok(0o60), 'x' => Ok(0o61), 'y' => Ok(0o62), 'z' => Ok(0o63),
        '0' => Ok(0o64), '1' => Ok(0o65), '2' => Ok(0o66), '3' => Ok(0o67),
        '4' => Ok(0o70), '5' => Ok(0o71), '6' => Ok(0o72), '7' => Ok(0o73),
        '8' => Ok(0o74), '9' => Ok(0o75), '+' => Ok(0o76), '/' => Ok(0o77),
         _  => Err(c)
    }
}

fn nibble_to_hexchar(i: u8) -> Result<char, u8> {
    let hexchars = "0123456789ABCDEF";

    hexchars.chars().nth(i as usize).ok_or(i)
}

fn nibble_to_base64char(i: u8) -> Result<char, u8> {
    let base64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                       abcdefghijklmnopqrstuvwxyz\
                       0123456789+/";

    base64chars.chars().nth(i as usize).ok_or(i)
}

fn nibbles_to_byte(nibbles: &[u8]) -> Option<u8> {
    match (nibbles.get(0), nibbles.get(1)) {
        (Some(a), Some(b)) => Some((a << 4) ^ b),
        (Some(_), None) => None,
        _ => panic!("unreachable")
    }
}

#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ByteString { pub bytes: Vec<u8> } 

impl ByteString {
    fn new(bytes: Vec<u8>) -> ByteString {
        ByteString { bytes: bytes }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> ByteString {
        ByteString { bytes: bytes }
    }

    pub fn as_utf8(&self) -> io::Result<String> {
        let mut string = String::new();

        try!(self.bytes.as_slice().read_to_string(&mut string));

        Ok(string)
    }

    pub fn from_hex(hex_string: &str) -> Result<ByteString, String> {
        let chars: Result<Vec<u8>, char> = hex_string.chars()
            .map(hexchar_to_nibble)
            .collect();

        match chars {
            Err(c) => {
                Err(format!("Not a hex character: {}", c))
            },
            Ok(chars) => {
                let chars: Option<Vec<u8>> = chars.chunks(2)
                    .map(nibbles_to_byte)
                    .collect();

                chars.ok_or("Hex string length must be even".to_string())
                    .map(ByteString::new)
            }
        }
    }

    pub fn from_base64(base64_string: &str) -> Result<ByteString, String> {
        let chars: Result<Vec<u8>, char> = base64_string.chars()
            .inspect(|c| println!("Char: {}", c))
            .map(base64char_to_sextet)
            .inspect(|i| println!("Sextet: {:?}", i))
            .filter(|i| match *i {
                Err('=') => false, // We don't care about these
                _ => true
            }).collect();

        match chars {
            Err(c) => {
                Err(format!("Not a Base 64 character: {}", c))
            },
            Ok(chars) => {
                if chars.len() % 4 == 1 {
                    Err("Base 64 string cannot have \
                         one dangling character".to_string())
                } else {
                    Ok(ByteString { bytes: chars.chunks(4)
                        .fold(Vec::new(), |mut vec, chunk| {
                            match (chunk.get(0), chunk.get(1),
                                   chunk.get(2), chunk.get(3)) {
                                (Some(a), Some(b), None, None) => {
                                    vec.push((a << 2) ^ (b >> 4));
                                    vec
                                },
                                (Some(a), Some(b), Some(c), None) => {
                                    vec.push((a << 2) ^ (b >> 4));
                                    vec.push(((b & 0x0F) << 4) ^ (c >> 2));
                                    vec
                                },
                                (Some(a), Some(b), Some(c), Some(d)) => {
                                    vec.push((a << 2) ^ (b >> 4));
                                    vec.push(((b & 0x0F) << 4) ^ (c >> 2));
                                    vec.push(((c & 0x03) << 6) ^ d);
                                    vec
                                },
                                _ => panic!("Unreachable")
                            }
                    })})
                }
            }
        }
    }

    pub fn to_hex(&self) -> String {
        self.bytes.clone().into_iter()
            .map(|b| format!("{}{}", 
                             nibble_to_hexchar(b >> 4).unwrap(), 
                             nibble_to_hexchar(b & 0xF).unwrap()))
            .join("")
    }

    pub fn to_base64(&self) -> String {
        self.bytes.chunks(3).map(|chunk|
            match (chunk.get(0), chunk.get(1), chunk.get(2)) {
                (Some(a), None, None) =>
                    format!("{}{}==",
                            nibble_to_base64char(a >> 2).unwrap(),
                            nibble_to_base64char((a & 0x03) << 4).unwrap()),
                (Some(a), Some(b), None) =>
                    format!("{}{}{}=",
                            nibble_to_base64char(a >> 2).unwrap(),
                            nibble_to_base64char(((a & 0x03) << 4) ^ (b >> 4)).unwrap(),
                            nibble_to_base64char((b & 0x0F) << 2).unwrap()),
                (Some(a), Some(b), Some(c)) =>
                    format!("{}{}{}{}",
                            nibble_to_base64char(a >> 2).unwrap(),
                            nibble_to_base64char(((a & 0x03) << 4) ^ (b >> 4)).unwrap(),
                            nibble_to_base64char(((b & 0x0F) << 2) ^ (c >> 6)).unwrap(),
                            nibble_to_base64char(c & 0x3F).unwrap()),
                _ => panic!("Unreachable")
            }).join("")
    }
}

#[cfg(test)]
mod tests {
    use super::ByteString;

    #[test]
    fn from_hex_test() {
        assert_eq!(
            ByteString::from_hex("48656C6C6F").unwrap(),
            ByteString{bytes: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]})
    }

    #[test]
    fn to_hex_test() {
        assert_eq!(
            ByteString{bytes: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]}.to_hex(),
            "48656C6C6F".to_string())
    }

    #[test]
    fn from_base64_test() {
        assert_eq!(
            ByteString::from_base64("SGVsbG8=").unwrap(),
            ByteString{bytes: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]})
    }

    #[test]
    fn to_base64_test() {
        assert_eq!(
            ByteString{bytes: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]}.to_base64(),
            "SGVsbG8=".to_string())
    }

    #[test]
    fn s1() {
        assert_eq!(
            ByteString::from_hex("49276d206b696c6c\
                                  696e6720796f7572\
                                  20627261696e206c\
                                  696b65206120706f\
                                  69736f6e6f757320\
                                  6d757368726f6f6d")
                .unwrap()
                .to_base64(), 
            "SSdtIGtpbGxpbmcg\
             eW91ciBicmFpbiBs\
             aWtlIGEgcG9pc29u\
             b3VzIG11c2hyb29t".to_string())
    }
}
