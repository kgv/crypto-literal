use crate::util::Serialize;
use generic_array::{typenum::U32, GenericArray};
use sha2::{Digest, Sha256};
use syn::{
    parse::{Parse, ParseStream},
    Lit, Result,
};

/// Key.
pub(crate) struct Key {
    literal: Lit,
}

impl Key {
    pub(crate) fn digest(&self) -> GenericArray<u8, U32> {
        let buffer = self.literal.serialize();
        Sha256::digest(&buffer)
    }
}

impl Parse for Key {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            literal: input.parse()?,
        })
    }
}
