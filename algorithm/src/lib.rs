use aes::Aes256;
use ofb::{
    stream_cipher::{NewStreamCipher, SyncStreamCipher},
    Ofb,
};

/// Crypto algorithm.
#[derive(Debug, PartialEq, Eq)]
pub enum Algorithm {
    Aes(Aes),
    Xor(Xor),
}

impl Algorithm {
    pub fn encrypt(&self, buffer: &mut [u8]) {
        match self {
            Algorithm::Aes(Aes { iv, key }) => Aes::encrypt(buffer, iv, key),
            Algorithm::Xor(Xor { key }) => Xor::encrypt(buffer, key),
        }
    }

    pub fn decrypt(&self, buffer: &mut [u8]) {
        match self {
            Algorithm::Aes(Aes { iv, key }) => Aes::decrypt(buffer, iv, key),
            Algorithm::Xor(Xor { key }) => Xor::decrypt(buffer, key),
        }
    }
}

/// Aes algorithm.
#[derive(Debug, PartialEq, Eq)]
pub struct Aes {
    pub iv: &'static [u8],
    pub key: &'static [u8],
}

impl Aes {
    #[inline(always)]
    pub fn encrypt(buffer: &mut [u8], iv: &[u8], key: &[u8]) {
        let mut cipher = Ofb::<Aes256>::new_var(key, iv).expect("create cipher (Ofb<Aes256>)");
        cipher.apply_keystream(buffer);
    }

    #[inline(always)]
    pub fn decrypt(buffer: &mut [u8], iv: &[u8], key: &[u8]) {
        Self::encrypt(buffer, iv, key);
    }
}

/// Xor algorithm.
#[derive(Debug, PartialEq, Eq)]
pub struct Xor {
    pub key: &'static [u8],
}

impl Xor {
    #[inline(always)]
    pub fn encrypt(buffer: &mut [u8], key: &[u8]) {
        buffer
            .iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(byte, key)| *byte ^= *key)
    }

    #[inline(always)]
    pub fn decrypt(buffer: &mut [u8], key: &[u8]) {
        Self::encrypt(buffer, key);
    }
}
