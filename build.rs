use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cpp/wrapper.h");
    println!("cargo:rerun-if-changed=cpp/lib.cpp");

    // Compile C++ library
    cc::Build::new()
        .cpp(true)
        .file("cpp/lib.cpp")
        .include("cpp")
        .flag_if_supported("-std=c++11")
        .compile("proxie_cpp");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    // Attempt to generate Rust bindings using bindgen if libclang is available
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let generated = std::panic::catch_unwind(|| {
        bindgen::Builder::default()
            .header("CPP/wrapper.h")
            .allowlist_function("str_length")
            .allowlist_function("str_reverse")
            .allowlist_function("count_vowels")
            .allowlist_function("to_uppercase")
            .allowlist_function("free_string")
            .generate()
    });
    std::panic::set_hook(prev_hook);

    match generated {
        Ok(Ok(bindings)) => {
            bindings
                .write_to_file(&bindings_path)
                .expect("Couldn't write bindings!");
        }
        _ => {
            // Fallback bindings when libclang is not installed
            let fallback_bindings = r#"
extern "C" {
    pub fn str_length(input: *const ::std::os::raw::c_char) -> usize;
    pub fn str_reverse(input: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn count_vowels(input: *const ::std::os::raw::c_char) -> usize;
    pub fn to_uppercase(input: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn free_string(ptr: *mut ::std::os::raw::c_char);
}
"#;
            std::fs::write(&bindings_path, fallback_bindings)
                .expect("Couldn't write fallback bindings!");
        }
    }
}
