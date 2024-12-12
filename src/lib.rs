use std::ops::RangeBounds;

pub mod slice;
pub mod slice_byte;
pub mod substring;
pub mod substring_byte;
pub mod truncate;
pub mod truncate_byte;

pub trait StringUtils {
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
    fn slice_byte(&self, range: impl RangeBounds<usize>) -> &str;

    fn substring(&self, start: usize, len: usize) -> &str;
    fn substring_byte(&self, start: usize, len: usize) -> &str;

    fn truncate(&self, len: usize) -> &str;
    fn truncate_byte(&self, len: usize) -> &str;
}

impl StringUtils for str {
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        slice::apply(self, range)
    }
    fn slice_byte(&self, range: impl RangeBounds<usize>) -> &str {
        slice_byte::apply(self, range)
    }

    fn substring(&self, start: usize, len: usize) -> &str {
        substring::apply(self, start, len)
    }
    fn substring_byte(&self, start: usize, len: usize) -> &str {
        substring_byte::apply(self, start, len)
    }

    fn truncate(&self, len: usize) -> &str {
        truncate::apply(self, len)
    }
    fn truncate_byte(&self, len: usize) -> &str {
        truncate_byte::apply(self, len)
    }
}
