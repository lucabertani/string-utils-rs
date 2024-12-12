use std::ops::{Bound, RangeBounds};

use crate::substring_byte;

pub(crate) fn apply(s: &str, range: impl RangeBounds<usize>) -> &str {
    let start = match range.start_bound() {
        Bound::Included(bound) | Bound::Excluded(bound) => *bound,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(bound) => *bound + 1,
        Bound::Excluded(bound) => *bound,
        Bound::Unbounded => s.len(),
    };

    if start > end {
        return "";
    }

    substring_byte::apply(s, start, end - start)
}
