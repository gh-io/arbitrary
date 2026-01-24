#![crate_type = "cdylib"]

use snap::raw::{Encoder, Decoder};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Compress input using Snappy
#[no_mangle]
pub extern "C" fn snappy_compress(input_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if input_ptr.is_null() {
            return std::ptr::null_mut();
        }

        // Convert C string to Rust string
        let c_str = CStr::from_ptr(input_ptr);
        let input_bytes = c_str.to_bytes();

        // Compress
        let mut encoder = Encoder::new();
        match encoder.compress_vec(input_bytes) {
            Ok(compressed) => {
                // Return as C string (base64 encoded so safe as text)
                let base64 = base64::encode(&compressed);
                CString::new(base64).unwrap().into_raw()
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Decompress input using Snappy
#[no_mangle]
pub extern "C" fn snappy_decompress(input_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if input_ptr.is_null() {
            return std::ptr::null_mut();
        }

        // Convert C string to Rust string
        let c_str = CStr::from_ptr(input_ptr);
        let base64_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        // Decode base64 and decompress
        let compressed_bytes = match base64::decode(base64_str) {
            Ok(b) => b,
            Err(_) => return std::ptr::null_mut(),
        };

        let mut decoder = Decoder::new();
        match decoder.decompress_vec(&compressed_bytes) {
            Ok(decompressed) => {
                CString::new(decompressed).unwrap().into_raw()
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Free memory allocated by Rust for C strings
#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s); } // drops the string
}

/// Example arithmetic function
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
