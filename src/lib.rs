//! Crypto literal
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! #
//! # use crypto_literal::encrypt;
//! #
//! let crypto_literal = encrypt!("The quick brown fox jumps over the lazy dog");
//! ```
//!
//! Static usage:
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! #
//! # use crypto_literal::{encrypt, CryptoLiteral};
//! # use lazy_static::lazy_static;
//! #
//! lazy_static! {
//!     static ref CRYPTO_LITERAL: CryptoLiteral<str> =
//!         encrypt!("The quick brown fox jumps over the lazy dog.");
//! }
//! ```

#![recursion_limit = "256"]

pub use self::literal::Literal;
pub use algorithm::Algorithm;
#[doc(inline)]
pub use crypto_literal_algorithm as algorithm;
#[doc(hidden)]
pub use crypto_literal_macro as r#macro;
pub use r#macro::{algorithm, encrypt, key};

use derive_more::{AsRef, Deref, Display, From, Into};
use serde::Deserialize;
use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};

/// Crypto literal.
#[derive(AsRef, Deref, Display, From, Into)]
#[as_ref(forward)]
#[deref(forward)]
#[display(fmt = "{}", _0)]
pub struct CryptoLiteral<T: Literal + ?Sized>(Cow<'static, T>);

impl<T: Literal + ?Sized> CryptoLiteral<T>
where
    <T as ToOwned>::Owned: for<'de> Deserialize<'de>,
{
    pub fn new(algorithm: Algorithm, mut buffer: Vec<u8>) -> Self {
        algorithm.decrypt(&mut buffer);
        let deserialized = bincode::deserialize(&buffer).expect("deserialize literal");
        Self(Cow::Owned(deserialized))
    }
}

impl<T> Debug for CryptoLiteral<T>
where
    T: Literal,
    T: Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CryptoLiteral").field(&self.0).finish()
    }
}

mod literal;
