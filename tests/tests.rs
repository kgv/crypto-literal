#![feature(proc_macro_hygiene)]

use anyhow::Result;
use crypto_literal::{algorithm::Xor, Algorithm, CryptoLiteral};

mod basic {
    use crypto_literal::encrypt;

    #[test]
    fn lit_str() {
        let crypto_literal = encrypt!("The quick brown fox jumps over the lazy dog.");
        assert_eq!(
            &*crypto_literal,
            "The quick brown fox jumps over the lazy dog."
        );
    }

    #[test]
    fn lit_byte_str() {
        let crypto_literal = encrypt!(b"The quick brown fox jumps over the lazy dog.");
        assert_eq!(
            &*crypto_literal,
            &b"The quick brown fox jumps over the lazy dog."[..]
        );
    }

    #[test]
    fn lit_byte() {
        let crypto_literal = encrypt!(b'g');
        assert_eq!(*crypto_literal, b'g');
    }

    #[test]
    fn lit_char() {
        let crypto_literal = encrypt!('g');
        assert_eq!(*crypto_literal, 'g');
    }

    #[test]
    fn lit_int() {
        let crypto_literal = encrypt!(0x09);
        assert_eq!(*crypto_literal, 0x09);
        let crypto_literal = encrypt!(0x09_u8);
        assert_eq!(*crypto_literal, 0x09_u8);
        let crypto_literal = encrypt!(0x09_u16);
        assert_eq!(*crypto_literal, 0x09_u16);
        let crypto_literal = encrypt!(0x09_u32);
        assert_eq!(*crypto_literal, 0x09_u32);
        let crypto_literal = encrypt!(0x09_u64);
        assert_eq!(*crypto_literal, 0x09_u64);
        let crypto_literal = encrypt!(0x09i8);
        assert_eq!(*crypto_literal, 0x09_i8);
        let crypto_literal = encrypt!(0x09_i16);
        assert_eq!(*crypto_literal, 0x09_i16);
        let crypto_literal = encrypt!(0x09_i32);
        assert_eq!(*crypto_literal, 0x09_i32);
        let crypto_literal = encrypt!(0x09_i64);
        assert_eq!(*crypto_literal, 0x09_i64);
    }

    #[test]
    fn lit_float() {
        let crypto_literal = encrypt!(9.0);
        assert_eq!(*crypto_literal, 9.0);
        let crypto_literal = encrypt!(9.0_f32);
        assert_eq!(*crypto_literal, 9.0_f32);
        let crypto_literal = encrypt!(9.0_f64);
        assert_eq!(*crypto_literal, 9.0_f64);
    }

    #[test]
    fn lit_bool() {
        let crypto_literal = encrypt!(false);
        assert_eq!(*crypto_literal, false);
        let crypto_literal = encrypt!(true);
        assert_eq!(*crypto_literal, true);
    }
}

mod lazy_static {
    use crypto_literal::{encrypt, CryptoLiteral};
    use lazy_static::lazy_static;

    #[test]
    fn lit_str() {
        lazy_static! {
            static ref CRYPTO_LITERAL: CryptoLiteral<str> =
                encrypt!("The quick brown fox jumps over the lazy dog.");
        }
        assert_eq!(
            &**CRYPTO_LITERAL,
            "The quick brown fox jumps over the lazy dog.",
        );
    }

    #[test]
    fn lit_byte_str() {
        lazy_static! {
            static ref CRYPTO_LITERAL: CryptoLiteral<[u8]> =
                encrypt!(b"The quick brown fox jumps over the lazy dog.");
        }
        assert_eq!(
            &**CRYPTO_LITERAL,
            &b"The quick brown fox jumps over the lazy dog."[..],
        );
    }

    #[test]
    fn lit_byte() {
        lazy_static! {
            static ref CRYPTO_LITERAL: CryptoLiteral<u8> = encrypt!(b'g');
        }
        assert_eq!(**CRYPTO_LITERAL, b'g');
    }

    #[test]
    fn lit_char() {
        lazy_static! {
            static ref CRYPTO_LITERAL: CryptoLiteral<char> = encrypt!('g');
        }
        assert_eq!(**CRYPTO_LITERAL, 'g');
    }
}

#[test]
fn without_macro() -> Result<()> {
    let key = b"0123456789";
    let algorithm = Algorithm::Xor(Xor { key });

    let source = "The quick brown fox jumps over the lazy dog.";
    let mut encoded = bincode::serialize("The quick brown fox jumps over the lazy dog.")?;
    Xor::encrypt(&mut encoded, key);
    let target = CryptoLiteral::<str>::new(algorithm, encoded);
    assert_eq!(source, &*target);
    Ok(())
}
