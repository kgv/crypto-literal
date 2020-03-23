use crate::{algorithm::Algorithm, util::Serialize, ALGORITHM, IV, KEY256};
use crypto_literal_algorithm::{Aes, Xor};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    Lit,
};

/// CryptoLiteral.
pub(crate) struct CryptoLiteral {
    literal: Lit,
}

impl Parse for CryptoLiteral {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            literal: input.parse()?,
        })
    }
}

impl CryptoLiteral {
    fn r#type(&self) -> TokenStream {
        match &self.literal {
            Lit::Str(_) => quote!(str),
            Lit::ByteStr(_) => quote!([u8]),
            Lit::Byte(_) => quote!(u8),
            Lit::Char(_) => quote!(char),
            Lit::Int(lit_int) => match lit_int.suffix() {
                "i8" => quote!(i8),
                "u8" => quote!(u8),
                "i16" => quote!(i16),
                "u16" => quote!(u16),
                "i32" => quote!(i32),
                "u32" => quote!(u32),
                "i64" => quote!(i64),
                "u64" => quote!(u64),
                "i128" => quote!(i128),
                "u128" => quote!(u128),
                "isize" => quote!(isize),
                "" | "usize" => quote!(usize),
                _ => unreachable!(),
            },
            Lit::Float(lit_float) => match lit_float.suffix() {
                "f32" => quote!(f32),
                "" | "f64" => quote!(f64),
                _ => unreachable!(),
            },
            Lit::Bool(_) => quote!(bool),
            _ => unimplemented!(),
        }
    }
}

impl ToTokens for CryptoLiteral {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let r#type = self.r#type();
        let mut buffer = self.literal.serialize();
        let algorithm = *ALGORITHM.lock().unwrap();
        match algorithm {
            // TODO: Algorithm contain IV and KEY
            Algorithm::Aes => Aes::encrypt(&mut buffer, &*IV, &*KEY256),
            Algorithm::Xor => Xor::encrypt(&mut buffer, &*KEY256),
        }
        let encrypt_tokens =
            quote!(crypto_literal::CryptoLiteral::<#r#type>::new(#algorithm, vec![#(#buffer),*]));
        encrypt_tokens.to_tokens(tokens);
    }
}
