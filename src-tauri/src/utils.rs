pub fn option_as_slice(value: &Option<String>) -> Option<&str> {
    if let Some(content) = value {
        return Some(&content[..]);
    }
    None
}

pub trait ToI64 {
    fn to_i64(&self) -> Option<i64>;
}

impl ToI64 for Option<i32> {
    fn to_i64(&self) -> Option<i64> {
        if let Some(content) = self {
            return Some(*content as i64);
        }
        None
    }
}

impl ToI64 for Option<u16> {
    fn to_i64(&self) -> Option<i64> {
        if let Some(content) = self {
            return Some(*content as i64);
        }
        None
    }
}
