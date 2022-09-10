use crate::dboilerplate::configuration::is_valid_identifier;

#[test]
fn test_is_valid_identifier() {
    assert_eq!(is_valid_identifier("valid_name"), true);
    static INVALID_CHARS: [char; 15] = [
        '\\', '/', '?', '%', '*', ':', '|', '"', '<', '>', '.', ',', ';', '=', ' ',
    ];
    for c in INVALID_CHARS.iter() {
        assert_eq!(is_valid_identifier(&format!("invalid{}", c)), false);
    }
}
