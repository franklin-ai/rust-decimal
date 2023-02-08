use num_traits::cast::{FromPrimitive, NumCast, ToPrimitive};
use crate::prelude::Decimal;
use std::any::{Any, type_name};


// By directly calling `type_name` here we guarantee that the names
// remain "up to date".
const I8_NAME: &str = type_name::<i8>();
const I16_NAME: &str = type_name::<i16>();
const I32_NAME: &str = type_name::<i32>();
const I64_NAME: &str = type_name::<i64>();
const I128_NAME: &str = type_name::<i128>();
const ISIZE_NAME: &str = type_name::<isize>();
const F32_NAME: &str = type_name::<f32>();
const F64_NAME: &str = type_name::<f64>();
const SIGNED_TYPES: [&str; 8] = [
    I8_NAME, I16_NAME, I32_NAME, I64_NAME,
    I128_NAME, ISIZE_NAME, F32_NAME, F64_NAME,
];

const fn const_cmp_str(lhs: &str, rhs: &str) -> bool {
    // Obviously the below code only makes sense in the context of
    // calling it at compile-time with constant input for both `lhs`
    // and `rhs`.
    let lhs_bytes = lhs.as_bytes();
    let rhs_bytes = rhs.as_bytes();
    if lhs_bytes.len() != rhs_bytes.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs_bytes.len() {
        if lhs_bytes[i] != rhs_bytes[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn is_signed_internal<T: ?Sized>() -> bool {
    let mut i = 0;
    // `for x in y` does not work in const contexts yet.
    while i < SIGNED_TYPES.len() {
        // Can't do `SIGNED_TYPES[i] == type_name::<T>()` and have this
        // be `const`.
        if const_cmp_str(SIGNED_TYPES[i], type_name::<T>()) {
            return true;
        }
        i += 1;
    }
    false
}

#[inline(always)]
pub const fn is_signed<T: ?Sized>() -> bool {
    is_signed_internal::<T>()
}

#[inline(always)]
pub const fn is_signed_value<T: ?Sized>(_val: &T) -> bool {
    is_signed_internal::<T>()
}


impl NumCast for Decimal {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        // Unwrap here safely as we have performed type assertion
        match _ {
            is_signed::<f32>() => Decimal::from_f32(n.to_f32().unwrap()),
            is_signed::<f64>() => Decimal::from_f64(n.to_f64().unwrap()),

            is_signed::<i8>() => Decimal::from_i8(n.to_i8().unwrap()),
            is_signed::<i16>() => Decimal::from_i16(n.to_i16().unwrap()),
            is_signed::<i32>() => Decimal::from_i32(n.to_i32().unwrap()),
            is_signed::<i64>() => Decimal::from_i64(n.to_i64().unwrap()),
            is_signed::<i128>() => Decimal::from_i128(n.to_i128().unwrap()),

            is_signed::<u8>() => Decimal::from_u8(n.to_u8().unwrap()),
            is_signed::<u32>() => Decimal::from_u32(n.to_u32().unwrap()),
            is_signed::<u64>() => Decimal::from_u64(n.to_u64().unwrap()),
            is_signed::<u128>() => Decimal::from_u128(n.to_u128().unwrap()),
        }

        // DEFAULT convert to 128 then to Decimal.
        let as_i128 = n.to_i128();
        if as_i128.is_none() {
            return None
        }
        FromPrimitive::from_i128(as_i128.unwrap())

    }
}


use geo::Coord;
pub fn t () {
    Coord::<Decimal>{x: 1., y:1.}
}
