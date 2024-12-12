use std::sync::LazyLock;

use regex::Regex;

static INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?\d+$").unwrap());
//static DECIMAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?\d+(\.\d+)?$").unwrap());

static DECIMAL: LazyLock<Regex> =
    //LazyLock::new(|| Regex::new(r"^-?(\d+\.\d+)|(\.\d+)?$").unwrap());
    LazyLock::new(|| Regex::new(r"^-?(\d+\.\d*)|(\.\d+)?$").unwrap());
static UNSIGNED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());

pub(crate) fn is_integer(s: &str) -> bool {
    INTEGER.is_match(s)
}
pub(crate) fn is_decimal(s: &str) -> bool {
    DECIMAL.is_match(s)
}
pub(crate) fn is_unsigned(s: &str) -> bool {
    UNSIGNED.is_match(s)
}
pub(crate) fn is_number(s: &str) -> bool {
    is_decimal(s)
}
