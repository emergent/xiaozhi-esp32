# Rust component for ESP-IDF
# This integrates a pre-built Rust library into the ESP-IDF build system
#
# IMPORTANT: Build the Rust library first with:
#   cd components/xiaozhi-rust
#   source ~/export-esp.sh
#   cargo build --release

# Get component directory (this file's location)
get_filename_component(COMPONENT_DIR "${CMAKE_CURRENT_LIST_FILE}" DIRECTORY)

# Get the target architecture from ESP-IDF
idf_build_get_property(idf_target IDF_TARGET)

# Map ESP-IDF target to Rust target triple
if(idf_target STREQUAL "esp32c3")
    set(RUST_TARGET "riscv32imc-esp-espidf")
elseif(idf_target STREQUAL "esp32c6")
    set(RUST_TARGET "riscv32imac-esp-espidf")
elseif(idf_target STREQUAL "esp32s3")
    set(RUST_TARGET "xtensa-esp32s3-espidf")
elseif(idf_target STREQUAL "esp32p4")
    set(RUST_TARGET "riscv32imafc-esp-espidf")
elseif(idf_target STREQUAL "esp32c5")
    set(RUST_TARGET "riscv32imac-esp-espidf")
else()
    message(FATAL_ERROR "Unsupported ESP-IDF target: ${idf_target}")
endif()

# Use release build by default
set(RUST_PROFILE "release")

# Rust library path (relative to component directory)
set(RUST_STATIC_LIBRARY "${COMPONENT_DIR}/target/${RUST_TARGET}/${RUST_PROFILE}/libxiaozhi_rust.a")

# Check if the Rust library exists
if(NOT EXISTS ${RUST_STATIC_LIBRARY})
    message(FATAL_ERROR
        "Rust library not found: ${RUST_STATIC_LIBRARY}\n"
        "Please build it first:\n"
        "  cd components/xiaozhi-rust\n"
        "  source ~/export-esp.sh\n"
        "  cargo build --release"
    )
endif()

message(STATUS "Using Rust library: ${RUST_STATIC_LIBRARY}")

# Register the component
idf_component_register(
    INCLUDE_DIRS "include"
)

# Add the prebuilt Rust library
add_prebuilt_library(xiaozhi_rust_prebuilt "${RUST_STATIC_LIBRARY}"
    PRIV_REQUIRES ""
)

# Link the Rust library to this component
target_link_libraries(${COMPONENT_LIB} INTERFACE xiaozhi_rust_prebuilt)
