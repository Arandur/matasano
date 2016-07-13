extern crate matasano;

extern crate rustc_serialize;

mod s1 {
    // Modules
    use matasano::english;
    use std::f64;
    use rustc_serialize::base64;

    // Structs
    use std::cmp::Ordering;

    // Traits
    use matasano::crypt::Crypt;
    use rustc_serialize::hex::FromHex;
    use rustc_serialize::base64::ToBase64;

    #[test]
    fn c1() {
        assert_eq!(
            "49276d206b696c6c\
             696e6720796f7572\
             20627261696e206c\
             696b65206120706f\
             69736f6e6f757320\
             6d757368726f6f6d".from_hex()
                .unwrap()
                .as_slice()
                .to_base64(base64::Config { 
                    char_set: base64::CharacterSet::Standard,
                    newline: base64::Newline::LF,
                    pad: true,
                    line_length: None}), 
            "SSdtIGtpbGxpbmcg\
             eW91ciBicmFpbiBs\
             aWtlIGEgcG9pc29u\
             b3VzIG11c2hyb29t".to_owned())
    }

    #[test]
    fn c2() {
        let b1     = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
        let b2     = "686974207468652062756c6c277320657965".from_hex().unwrap();
        let result = "746865206b696420646f6e277420706c6179".from_hex().unwrap();

        assert_eq!(b1.xor_encrypt_with(&b2), result);
    }

    #[test]
    fn c3() {
        let crypttext = "1b37373331363f78\
                         151b7f2b78343133\
                         3d78397828372d36\
                         3c78373e783a393b\
                         3736".from_hex().unwrap();

        let plaintext = (0..256).map(|i| vec![i as u8])
            .map(|k| crypttext.xor_decrypt_with(&k))
            .map(|v| String::from_utf8(v))
            .filter(|r| r.is_ok()).map(|r| r.unwrap())
            .fold((String::default(), f64::INFINITY), |acc, s| {
                let s_score = english::english_score(&s);

                match s_score.partial_cmp(&acc.1) {
                    Some(Ordering::Less) => (s, s_score),
                    _ => acc
                }
            }).0;

        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }
}
