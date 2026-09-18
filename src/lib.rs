use std::ffi::{CStr, CString};

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub enum FfiError {
    InvalidInput,
    NullPointer,
    InvalidUtf8,
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiError::InvalidInput => write!(f, "Input contains an invalid null character"),

            FfiError::NullPointer => write!(f, "C++ returned a null pointer"),

            FfiError::InvalidUtf8 => write!(f, "C++ returned invalid UTF-8"),
        }
    }
}

impl std::error::Error for FfiError {}

/// Returns the length of a string.
pub fn str_length(input: &str) -> usize {
    let c_string = CString::new(input).expect("input contains an invalid null character");

    unsafe { ffi::str_length(c_string.as_ptr()) }
}

/// Reverses a string using the C++ implementation.
pub fn reverse_string(input: &str) -> Result<String, FfiError> {
    let c_string = CString::new(input).map_err(|_| FfiError::InvalidInput)?;

    let result_ptr = unsafe { ffi::str_reverse(c_string.as_ptr()) };

    if result_ptr.is_null() {
        return Err(FfiError::NullPointer);
    }

    let result = unsafe {
        CStr::from_ptr(result_ptr)
            .to_str()
            .map_err(|_| FfiError::InvalidUtf8)?
            .to_owned()
    };

    unsafe {
        ffi::free_string(result_ptr);
    }

    Ok(result)
}

/// Counts vowels in a string.
pub fn count_vowels(input: &str) -> usize {
    let c_string = CString::new(input).expect("input contains an invalid null character");

    unsafe { ffi::count_vowels(c_string.as_ptr()) }
}

/// Converts a string to uppercase.
pub fn to_uppercase(input: &str) -> Result<String, FfiError> {
    let c_string = CString::new(input).map_err(|_| FfiError::InvalidInput)?;

    let result_ptr = unsafe { ffi::to_uppercase(c_string.as_ptr()) };

    if result_ptr.is_null() {
        return Err(FfiError::NullPointer);
    }

    let result = unsafe {
        CStr::from_ptr(result_ptr)
            .to_str()
            .map_err(|_| FfiError::InvalidUtf8)?
            .to_owned()
    };

    unsafe {
        ffi::free_string(result_ptr);
    }

    Ok(result)
}
