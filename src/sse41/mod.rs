pub use super::*;
use core::mem;

mod overloads;
mod sse41;
pub use self::overloads::*;
pub use self::sse2::*;
pub use self::sse41::*;

#[derive(Copy, Debug, Clone)]
pub struct I64x2_41(pub __m128i);
impl SimdBase<I64x2_41, i64> for I64x2_41 {}
#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
impl SimdBase<I32x4_41, i32> for I32x4_41 {}
impl SimdSmallInt<I32x4_41, i32> for I32x4_41 {}
