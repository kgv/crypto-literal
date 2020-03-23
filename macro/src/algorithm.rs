use crate::{IV, KEY256};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// Algorithm.
#[derive(Clone, Copy)]
pub(crate) enum Algorithm {
    Aes,
    Xor,
}

impl Parse for Algorithm {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let algorithm = if lookahead.peek(self::kw::aes) {
            let _ = input.parse::<self::kw::aes>()?;
            Self::Aes
        } else if lookahead.peek(self::kw::xor) {
            let _ = input.parse::<self::kw::xor>()?;
            Self::Xor
        } else {
            return Err(lookahead.error());
        };
        Ok(algorithm)
    }
}

impl ToTokens for Algorithm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let algorithm_tokens = match self {
            Algorithm::Aes => {
                let iv = &IV;
                let key = &KEY256;
                quote! {{
                    use crypto_literal::{Algorithm, algorithm::Aes};

                    Algorithm::Aes(Aes {
                        iv: &[#(#iv),*],
                        key: &[#(#key),*],
                    })
                }}
            }
            Algorithm::Xor => {
                let key = &KEY256;
                quote! {{
                    use crypto_literal::{Algorithm, algorithm::Xor};

                    Algorithm::Xor(Xor {
                        key: &[#(#key),*],
                    })
                }}
            }
        };
        algorithm_tokens.to_tokens(tokens);
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(aes);
    custom_keyword!(xor);
}
