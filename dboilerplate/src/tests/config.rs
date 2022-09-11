use crate::util::configuration::is_valid_identifier;

#[test]
fn test_invalid_identifiers() {
    static INVALID_CHARS: [char; 15] = [
        '\\', '/', '?', '%', '*', ':', '|', '"', '<', '>', '.', ',', ';', '=', ' ',
    ];
    for c in INVALID_CHARS.iter() {
        assert_eq!(is_valid_identifier(&format!("invalid{}", c)), false);
    }
}

#[test]
fn test_valid_identifiers() {
    assert_eq!(is_valid_identifier("valid_name"), true);
}
