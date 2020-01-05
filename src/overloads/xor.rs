use super::*;
// -- Bitwise XOr
impl BitXor for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn bitxor(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 ^ rhs.0)
    }
}

impl BitXor for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn bitxor(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 ^ rhs.0)
    }
}

impl BitXor for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn bitxor(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 ^ rhs.0)
    }
}

impl BitXor for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn bitxor(self, rhs: F32x1) -> F32x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        F32x1(f32::from_bits(result))
    }
}

impl BitXor for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn bitxor(self, rhs: F64x1) -> F64x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        F64x1(f64::from_bits(result))
    }
}

impl BitXor for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn bitxor(self, rhs: I16x16) -> I16x16 {
        I16x16(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}

impl BitXor for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn bitxor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn bitxor(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}

impl BitXor for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn bitxor(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn bitxor(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}

impl BitXor for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn bitxor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}

impl BitXor for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn bitxor(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_xor_pd(self.0, rhs.0) })
    }
}
