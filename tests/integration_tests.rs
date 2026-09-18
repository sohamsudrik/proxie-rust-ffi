use proxie_rust_ffi::{count_vowels, reverse_string, str_length, to_uppercase};

#[test]
fn test_string_length() {
    assert_eq!(str_length("Hello"), 5);

    assert_eq!(str_length("Rust"), 4);
}

#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("Hello").unwrap(), "olleH");

    assert_eq!(reverse_string("Rust").unwrap(), "tsuR");
}

#[test]
fn test_count_vowels() {
    assert_eq!(count_vowels("Hello"), 2);

    assert_eq!(count_vowels("Beautiful"), 5);
}

#[test]
fn test_uppercase() {
    assert_eq!(to_uppercase("hello").unwrap(), "HELLO");

    assert_eq!(
        to_uppercase("Rust Programming").unwrap(),
        "RUST PROGRAMMING"
    );
}

#[test]
fn test_empty_string() {
    assert_eq!(str_length(""), 0);

    assert_eq!(count_vowels(""), 0);

    assert_eq!(reverse_string("").unwrap(), "");

    assert_eq!(to_uppercase("").unwrap(), "");
}

#[test]
fn test_single_character() {
    assert_eq!(str_length("A"), 1);

    assert_eq!(count_vowels("A"), 1);

    assert_eq!(reverse_string("A").unwrap(), "A");
}

#[test]
fn test_special_characters() {
    assert_eq!(str_length("Hello!"), 6);

    assert_eq!(count_vowels("Hello!"), 2);
}
