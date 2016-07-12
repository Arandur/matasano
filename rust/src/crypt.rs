use bytestring::ByteString;

pub trait Crypt<KEY = Self> {
    type Output;

    fn xor_encrypt_with(&self, key: KEY) -> Self::Output;

    fn xor_decrypt_with(&self, key: KEY) -> Self::Output {
        self.xor_encrypt_with(key)
    }
}

impl Crypt for ByteString {
    type Output = ByteString;

    fn xor_encrypt_with(&self, key: ByteString) -> ByteString {
        ByteString { bytes: self.bytes.iter()
            .zip(key.bytes.into_iter())
            .map(|(a, b)| a ^ b)
            .collect()}
    }
}

#[cfg(test)]
mod tests {
    use bytestring::ByteString;
    use super::Crypt;
    use english::english_score;

    #[test]
    fn s2() {
        let b1 = ByteString::from_hex("1c0111001f01010006\
                                       1a024b53535009181c")
            .unwrap();
        let b2 = ByteString::from_hex("686974207468652062\
                                       756c6c277320657965")
            .unwrap();

        assert_eq!(
            b1.xor_encrypt_with(b2).to_hex().to_lowercase(),
            "746865206b696420646f6e277420706c6179".to_string())
    }

    #[test]
    fn s3() {
        let crypttext = ByteString::from_hex("1b37373331363f78\
                                              151b7f2b78343133\
                                              3d78397828372d36\
                                              3c78373e783a393b3736")
            .unwrap();

        let plaintext = (0..256).map(|i| ByteString::from_bytes(vec![i as u8]))
            .map(|k| crypttext.xor_decrypt_with(k))
            .map(|bs| bs.as_utf8().unwrap())
            .map(|s| english_score(&s).unwrap())
            .fold(::std::f64::MAX, |acc, x| acc.min(x));

        format!("{}", plaintext);
    }
}
