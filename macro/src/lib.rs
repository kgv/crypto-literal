#![feature(const_type_name)]
#![recursion_limit = "256"]

use crate::{algorithm::Algorithm, crypto_literal::CryptoLiteral, key::Key};
use generic_array::{
    typenum::{U16, U24, U32},
    GenericArray,
};
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use rand::{thread_rng, Rng};
use std::sync::Mutex;
use syn::parse_macro_input;

lazy_static! {
    static ref ALGORITHM: Mutex<Algorithm> = Mutex::new(Algorithm::Xor);
    static ref IV: GenericArray<u8, U16> = {
        let mut buffer = [0; 16];
        thread_rng().fill(&mut buffer[..]);
        buffer.into()
    };
    static ref KEY: Mutex<GenericArray<u8, U32>> = {
        let mut buffer = [0; 32];
        thread_rng().fill(&mut buffer[..]);
        Mutex::new(buffer.into())
    };
    static ref KEY128: GenericArray<u8, U16> = {
        let key = KEY.lock().unwrap();
        GenericArray::clone_from_slice(&key[0..16])
    };
    static ref KEY192: GenericArray<u8, U24> = {
        let key = KEY.lock().unwrap();
        GenericArray::clone_from_slice(&key[0..24])
    };
    static ref KEY256: GenericArray<u8, U32> = {
        let key = KEY.lock().unwrap();
        GenericArray::clone_from_slice(&key)
    };
}

/// Algorithm.
#[proc_macro_attribute]
pub fn algorithm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let algorithm_tokens = parse_macro_input!(attr as Algorithm);
    {
        let mut algorithm = ALGORITHM.lock().unwrap();
        *algorithm = algorithm_tokens;
    }
    item
}

/// Key.
#[proc_macro_attribute]
pub fn key(attr: TokenStream, item: TokenStream) -> TokenStream {
    let key_tokens = parse_macro_input!(attr as Key);
    {
        let mut key = KEY.lock().unwrap();
        *key = key_tokens.digest();
    }
    item
}

/// Encrypt.
#[proc_macro]
pub fn encrypt(item: TokenStream) -> TokenStream {
    let crypto_literal_tokens = parse_macro_input!(item as CryptoLiteral);
    TokenStream::from(quote!(#crypto_literal_tokens))
}

mod algorithm;
mod crypto_literal;
mod key;
mod util;
