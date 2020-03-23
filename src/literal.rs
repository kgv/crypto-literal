/// Literal.
pub trait Literal: 'static + ToOwned {}

impl Literal for str {}

impl Literal for [u8] {}

impl Literal for char {}

impl Literal for bool {}

impl Literal for u8 {}

impl Literal for u16 {}

impl Literal for u32 {}

impl Literal for u64 {}

impl Literal for u128 {}

impl Literal for usize {}

impl Literal for i8 {}

impl Literal for i16 {}

impl Literal for i32 {}

impl Literal for i64 {}

impl Literal for i128 {}

impl Literal for isize {}

impl Literal for f32 {}

impl Literal for f64 {}
