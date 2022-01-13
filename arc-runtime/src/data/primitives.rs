pub use i128;
pub use i16;
pub use i32;
pub use i64;
pub use i8;

pub use u128;
pub use u16;
pub use u32;
pub use u64;
pub use u8;

pub use f32;
pub use f64;

pub use bool;
pub use char;

#[allow(non_camel_case_types)]
pub type unit = ();

use crate::data::conversions::IntoSendable;
use crate::data::conversions::IntoSharable;

macro_rules! convert_reflexive {
    { $ty:ty } => {
        impl IntoSendable for $ty {
            type T = Self;
            fn into_sendable(self) -> Self { self }
        }
        impl IntoSharable for $ty {
            type T = Self;
            fn into_sharable(self) -> Self { self }
        }
    }
}

convert_reflexive!(i8);
convert_reflexive!(i16);
convert_reflexive!(i32);
convert_reflexive!(i64);
convert_reflexive!(i128);

convert_reflexive!(u8);
convert_reflexive!(u16);
convert_reflexive!(u32);
convert_reflexive!(u64);
convert_reflexive!(u128);

convert_reflexive!(f32);
convert_reflexive!(f64);

convert_reflexive!(bool);
convert_reflexive!(char);

convert_reflexive!(unit);
