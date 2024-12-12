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

    while byte_target != target {
        if let Some(c) = it.next() {
            let s = byte_target + c.len_utf8();
            if byte_target + c.len_utf8() <= target {
                byte_target = s;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    byte_target
}

/*
pub(crate) fn apply(s: &str, start: usize, len: usize) -> &str {
    let mut it = s.chars();

    let byte_start = compute_index(&mut it, 0, start);
    let byte_end = compute_index(&mut it, byte_start, len);

    &s[byte_start..byte_end]
}

fn compute_index(it: &mut Chars<'_>, start: usize, target: usize) -> usize {
    let mut byte_target = start;
    let target = start + target;

    while byte_target != target {
        if let Some(c) = it.next() {
            let s = byte_target + c.len_utf8();
            if byte_target + c.len_utf8() <= target {
                byte_target = s;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    byte_target
}
*/
