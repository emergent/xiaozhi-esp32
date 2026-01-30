#ifndef XIAOZHI_RUST_H
#define XIAOZHI_RUST_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Initialize the Rust runtime and logging
 *
 * This should be called early in the application lifecycle,
 * before any other Rust functions are invoked.
 * Must be called from the main task only once.
 */
void xiaozhi_rust_init(void);

/**
 * @brief Get the Rust component version
 *
 * @return Null-terminated string with the version
 */
const char* xiaozhi_rust_version(void);

/**
 * @brief Calculate a simple additive checksum for ASCII text
 *
 * This function sums up all character values in the provided text
 * and returns the result as a uint8_t (with overflow wrapping).
 *
 * @param text Pointer to the text buffer
 * @param length Length of the text (excluding null terminator)
 * @return uint8_t checksum value
 */
uint8_t xiaozhi_rust_calculate_text_checksum(const char* text, size_t length);

#ifdef __cplusplus
}
#endif

#endif // XIAOZHI_RUST_H
