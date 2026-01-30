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

#ifdef __cplusplus
}
#endif

#endif // XIAOZHI_RUST_H
