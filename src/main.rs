use proxie_rust_ffi::{count_vowels, reverse_string, str_length, to_uppercase};

fn main() {
    println!("==================================================");
    println!("      Proxie C++ Library via Rust FFI Demo        ");
    println!("==================================================");

    let test_strings = [
        "Hello, World!",
        "Rust and C++ FFI Integration",
        "Quick brown fox jumps over the lazy dog",
    ];

    for input in &test_strings {
        println!("\n[Input]: \"{}\"", input);
        println!("  - String Length:  {}", str_length(input));
        println!("  - Vowel Count:    {}", count_vowels(input));

        match reverse_string(input) {
            Ok(reversed) => println!("  - Reversed:       \"{}\"", reversed),
            Err(err) => eprintln!("  - Reversed Error: {}", err),
        }

        match to_uppercase(input) {
            Ok(upper) => println!("  - Uppercase:      \"{}\"", upper),
            Err(err) => eprintln!("  - Uppercase Error:{}", err),
        }
    }

    println!("\n==================================================");
    println!("All C++ functions executed successfully via FFI!");
    println!("==================================================");
}

