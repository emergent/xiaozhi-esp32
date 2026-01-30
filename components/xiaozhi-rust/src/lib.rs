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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version_ptr = xiaozhi_rust_version();
        assert!(!version_ptr.is_null());
    }
}
