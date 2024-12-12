use crate::substring;

pub(crate) fn apply(s: &str, len: usize) -> &str {
    if len == 0 {
        return "";
    }
    if len > s.bytes().len() {
        return s;
    }
    substring::apply(s, 0, len)
}
