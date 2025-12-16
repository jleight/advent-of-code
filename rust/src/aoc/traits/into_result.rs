use crate::aoc::Result;

pub trait IntoResult {
    fn into_result(self) -> Result<Self>
    where
        Self: Sized;
}

impl<T> IntoResult for T {
    fn into_result(self) -> Result<T> {
        Ok(self)
    }
}
