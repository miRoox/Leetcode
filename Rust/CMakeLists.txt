add_custom_target(Rust
    ALL
    COMMAND cargo build --workspace --verbose
    WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)
add_test(
    NAME Rust 
    COMMAND cargo test --workspace
    WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)
