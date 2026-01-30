//! XiaoZhi Rust Components
//!
//! This library provides Rust implementations of XiaoZhi components,
//! gradually replacing the C++ codebase while maintaining compatibility
//! through FFI (Foreign Function Interface).

#![allow(unused)]

// Re-export commonly used types
pub use esp_idf_svc::hal;
pub use esp_idf_svc::sys;

/// Initialize the Rust runtime and logging
///
/// This should be called early in the application lifecycle,
/// before any other Rust functions are invoked.
///
/// # Safety
///
/// This function must be called from the main task only once.
#[no_mangle]
pub extern "C" fn xiaozhi_rust_init() {
    // Initialize ESP-IDF logging
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("XiaoZhi Rust runtime initialized");
}

/// Get the Rust component version
///
/// Returns a null-terminated C string with the version.
#[no_mangle]
pub extern "C" fn xiaozhi_rust_version() -> *const core::ffi::c_char {
    const VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const core::ffi::c_char
}

/// Calculate a simple additive checksum for ASCII text
///
/// This function sums up all character values in the provided text
/// and returns the result as a u8 (with overflow wrapping).
///
/// # Arguments
///
/// * `text` - Pointer to a null-terminated C string
/// * `length` - Length of the string (excluding null terminator)
///
/// # Returns
///
/// A u8 checksum value
///
/// # Safety
///
/// The caller must ensure that:
/// - `text` is a valid pointer to a buffer of at least `length` bytes
/// - The buffer contains valid UTF-8 data (or at least valid ASCII)
/// - The pointer remains valid for the duration of this call
#[no_mangle]
pub extern "C" fn xiaozhi_rust_calculate_text_checksum(
    text: *const core::ffi::c_char,
    length: usize,
) -> u8 {
    if text.is_null() {
        return 0;
    }

    unsafe {
        let slice = core::slice::from_raw_parts(text as *const u8, length);
        calculate_text_checksum_internal(slice)
    }
}

/// Internal implementation of text checksum calculation
///
/// This is the pure Rust implementation that's easy to test.
fn calculate_text_checksum_internal(text: &[u8]) -> u8 {
    text.iter()
        .fold(0u8, |checksum, &byte| checksum.wrapping_add(byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version_ptr = xiaozhi_rust_version();
        assert!(!version_ptr.is_null());
    }

    #[test]
    fn test_calculate_text_checksum_empty() {
        assert_eq!(calculate_text_checksum_internal(b""), 0);
    }

    #[test]
    fn test_calculate_text_checksum_single_char() {
        assert_eq!(calculate_text_checksum_internal(b"A"), 65); // ASCII 'A' = 65
    }

    #[test]
    fn test_calculate_text_checksum_multiple_chars() {
        // "ABC" = 65 + 66 + 67 = 198
        assert_eq!(calculate_text_checksum_internal(b"ABC"), 198);
    }

    #[test]
    fn test_calculate_text_checksum_overflow() {
        // 255 + 1 = 0 (with wrapping)
        assert_eq!(calculate_text_checksum_internal(&[255, 1]), 0);
    }

    #[test]
    fn test_calculate_text_checksum_typical() {
        // Typical usage with a short message
        let text = b"Hello";
        // H(72) + e(101) + l(108) + l(108) + o(111) = 500 % 256 = 244
        assert_eq!(calculate_text_checksum_internal(text), 244);
    }
}
