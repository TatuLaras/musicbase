pub fn option_as_slice(value: &Option<String>) -> Option<&str> {
    if let Some(content) = value {
        return Some(&content[..]);
    }
    None
}

pub fn option_cast<T, U>(value: Option<T>) -> Option<U>
where
    Option<T>: IntoOption<U>,
{
    value.into_option()
}

pub trait IntoOption<T> {
    fn into_option(&self) -> Option<T>;
}

impl IntoOption<i64> for Option<u16> {
    fn into_option(&self) -> Option<i64> {
        if let Some(content) = self {
            Some(*content as i64)
        } else {
            None
        }
    }
}

impl IntoOption<i64> for Option<i32> {
    fn into_option(&self) -> Option<i64> {
        if let Some(content) = self {
            Some(*content as i64)
        } else {
            None
        }
    }
}

impl IntoOption<String> for Option<&str> {
    fn into_option(&self) -> Option<String> {
        if let Some(content) = self {
            return Some(content.to_string());
        }
        None
    }
}

impl IntoOption<u16> for Option<i64> {
    fn into_option(&self) -> Option<u16> {
        if let Some(content) = self {
            Some(*content as u16)
        } else {
            None
        }
    }
}
