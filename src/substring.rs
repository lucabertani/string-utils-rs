use std::str::Chars;

pub(crate) fn apply(s: &str, start: usize, len: usize) -> &str {
    if len == 0 {
        return "";
    }

    let mut it = s.chars();

    let byte_start = compute_index(&mut it, start);
    let byte_end = byte_start + compute_index(&mut it, len);

    &s[byte_start..byte_end]
}

fn compute_index(it: &mut Chars<'_>, target: usize) -> usize {
    let mut byte_target = 0;
    let mut char_pos = 0;
    let target = target;

    while char_pos != target {
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_target += c.len_utf8();
        } else {
            break;
        }
    }

    byte_target
}
