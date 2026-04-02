use std::ffi::{CStr, c_char};

pub trait ToStr {
    fn to_str<'a>(&self) -> Option<&'a str>;
}

impl ToStr for *const c_char {
    fn to_str<'a>(&self) -> Option<&'a str> {
        if self.is_null() {
            return None;
        }
        let c_str = unsafe { CStr::from_ptr(*self) };
        c_str.to_str().ok()
    }
}

pub trait ToBool {
    fn to_bool(&self) -> Option<bool>;
}

impl ToBool for c_char {
    fn to_bool(&self) -> Option<bool> {
        match self {
            0 => Some(true),
            1 => Some(false),
            _ => None,
        }
    }
}

impl ToBool for i32 {
    fn to_bool(&self) -> Option<bool> {
        match self {
            0 => Some(true),
            1 => Some(false),
            _ => None,
        }
    }
}

impl ToBool for u8 {
    fn to_bool(&self) -> Option<bool> {
        match self {
            0 => Some(true),
            1 => Some(false),
            _ => None,
        }
    }
}

impl ToBool for *const u8 {
    fn to_bool(&self) -> Option<bool> {
        if self.is_null() {
            return None;
        }
        match unsafe { **self } {
            0 => Some(true),
            1 => Some(false),
            _ => None,
        }
    }
}
