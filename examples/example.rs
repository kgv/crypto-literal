#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]
// #![crypto_literal::algorithm(none)]
// #![crypto_literal::algorithm(xor)]
#![crypto_literal::algorithm(aes)]
#![crypto_literal::key("0123456789")]

use anyhow::Result;
use crypto_literal::{encrypt, CryptoLiteral};
use lazy_static::lazy_static;

lazy_static! {
    static ref CRYPTO_LITERAL: CryptoLiteral<str> =
        encrypt!("The quick brown fox jumps over the lazy dog.");
}

fn main() -> Result<()> {
    println!("CRYPTO_LITERAL: {:?}", &**CRYPTO_LITERAL);

    let crypto_literal = encrypt!("The quick brown fox jumps over the lazy dog.");
    println!("-crypto_literal: {:?}", &*crypto_literal);

    let crypto_literal = encrypt!(0xffff_0000_ffff_0000_u64);
    println!("-crypto_literal: {:x?}", &*crypto_literal);
    Ok(())
}
