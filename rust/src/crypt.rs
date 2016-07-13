pub trait Crypt {
    type Output;

    fn xor_encrypt_with(&self, key: &[u8]) -> Self::Output;

    fn xor_decrypt_with(&self, key: &[u8]) -> Self::Output {
        self.xor_encrypt_with(key)
    }
}

impl Crypt for Vec<u8> {
    type Output = Vec<u8>;

    fn xor_encrypt_with(&self, key: &[u8]) -> Vec<u8> {
        self.iter()
            .zip(key.into_iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect()
    }
}
