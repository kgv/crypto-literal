use anyhow::Context;
use std::{any::type_name, fmt::Display, str::FromStr};
use syn::{Lit, LitFloat, LitInt};

pub(crate) trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for Lit {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Lit::Str(lit_str) => bincode::serialize(&lit_str.value()),
            Lit::ByteStr(lit_byte_str) => bincode::serialize(&lit_byte_str.value()),
            Lit::Byte(lit_byte) => bincode::serialize(&lit_byte.value()),
            Lit::Char(lit_char) => bincode::serialize(&lit_char.value()),
            Lit::Int(lit_int) => match lit_int.suffix() {
                "i8" => bincode::serialize(&lit_int.parse::<i8>()),
                "u8" => bincode::serialize(&lit_int.parse::<u8>()),
                "i16" => bincode::serialize(&lit_int.parse::<i16>()),
                "u16" => bincode::serialize(&lit_int.parse::<u16>()),
                "i32" => bincode::serialize(&lit_int.parse::<i32>()),
                "u32" => bincode::serialize(&lit_int.parse::<u32>()),
                "i64" => bincode::serialize(&lit_int.parse::<i64>()),
                "u64" => bincode::serialize(&lit_int.parse::<u64>()),
                "i128" => bincode::serialize(&lit_int.parse::<i128>()),
                "u128" => bincode::serialize(&lit_int.parse::<u128>()),
                "isize" => bincode::serialize(&lit_int.parse::<isize>()),
                "" | "usize" => bincode::serialize(&lit_int.parse::<usize>()),
                _ => unreachable!(),
            },
            Lit::Float(lit_float) => match lit_float.suffix() {
                "f32" => bincode::serialize(&lit_float.parse::<f32>()),
                "" | "f64" => bincode::serialize(&lit_float.parse::<f64>()),
                _ => unreachable!(),
            },
            Lit::Bool(lit_bool) => bincode::serialize(&lit_bool.value),
            _ => unimplemented!(),
        }
        .expect("serialize literal")
    }
}

trait Parse {
    fn parse<T>(&self) -> T
    where
        T: FromStr,
        T::Err: Display;
}

impl Parse for LitInt {
    fn parse<T>(&self) -> T
    where
        T: FromStr,
        T::Err: Display,
    {
        self.base10_parse::<T>()
            .with_context(|| format!("(literal: {}, type: {})", self, type_name::<T>()))
            .expect("parse integer literal")
    }
}

impl Parse for LitFloat {
    fn parse<T>(&self) -> T
    where
        T: FromStr,
        T::Err: Display,
    {
        self.base10_parse::<T>()
            .with_context(|| format!("(literal: {}, type: {})", self, type_name::<T>()))
            .expect(&format!("parse float literal {}", type_name::<T>()))
    }
}
