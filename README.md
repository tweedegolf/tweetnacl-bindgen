# tweetnacl-bindgen

Making rust bindings for the [tweetnacl]() C library

Prerequisites:

- a C compiler is installed on the system
- bindgen, install with `cargo install bindgen`

Steps

0. Run `cargo new tweetnacl-bindgen`, then `cd tweetnacl-bindgen`
1. Copy `tweetnacl.h` and `tweetnacl.c` 
2. Create the rust bindings: `bindgen tweetnacl.h -o src/bindings.rs`
3. Use `build.rs` to compile and link `tweetnacl.c`. Create `build.rs` and insert
    ```rust
    fn main() {
        cc::Build::new()
            .file("tweetnacl.c")
            .compile("tweetnacl");   // outputs `libtweetnacl.a`
    }
    ```

    And add this section to your `Cargo.toml`

    ```toml
    [build-dependencies]
    cc = "1"
    ```
4. Run `cargo check` to verify everything is compiling correctly.
