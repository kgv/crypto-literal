# Crypto literal library

This crate has POC (Proof Of Concept) status!

## Supported algorithms

- Aes
- Xor

## Usage

First add `crypto-literal` crate to your `Cargo.toml`:

```toml
[dependencies]
crypto-literal = "0.1"
```

Now you can write the following code:

```rust
#![feature(proc_macro_hygiene)]

use crypto_literal::encrypt;

let crypto_literal = encrypt!("The quick brown fox jumps over the lazy dog.");
```

or:

```rust
#![feature(proc_macro_hygiene)]

use crypto_literal::{encrypt, CryptoLiteral};
use lazy_static::lazy_static;

lazy_static! {
    static ref CRYPTO_LITERAL: CryptoLiteral<str> =
        encrypt!("The quick brown fox jumps over the lazy dog.");
}
```
