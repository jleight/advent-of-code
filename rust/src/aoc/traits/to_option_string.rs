use crate::aoc::Result;

pub trait ToOptionString {
    fn to_option_string(&self) -> Option<String>;
}

macro_rules! to_option_string {
    ($type:tt) => {
        impl ToOptionString for $type {
            fn to_option_string(&self) -> Option<String> {
                Some(self.to_string())
            }
        }
    };
}

to_option_string!(i8);
to_option_string!(i16);
to_option_string!(i32);
to_option_string!(i64);
to_option_string!(i128);
to_option_string!(isize);
to_option_string!(u8);
to_option_string!(u16);
to_option_string!(u32);
to_option_string!(u64);
to_option_string!(u128);
to_option_string!(usize);
to_option_string!(f32);
to_option_string!(f64);

impl<T: ToOptionString> ToOptionString for Result<T> {
    fn to_option_string(&self) -> Option<String> {
        self.as_ref()
            .map_or(None, ToOptionString::to_option_string)
    }
}
