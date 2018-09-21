use utils::cli::*;

#[test]
fn test_cli_validator_usize_1() {
    match cli_validator_usize("".to_string()) {
        Err(e) => assert_eq!(&e, "cannot parse integer from empty string"),
        _ => assert!(false),
    }
}

#[test]
fn test_cli_validator_usize_2() {
    match cli_validator_usize("aa".to_string()) {
        Err(e) => assert_eq!(&e, "invalid digit found in string"),
        _ => assert!(false),
    }
}

#[test]
fn test_cli_validator_usize_3() {
    match cli_validator_usize("-42".to_string()) {
        Err(e) => assert_eq!(e, "invalid digit found in string"),
        _ => assert!(false),
    }
}

#[test]
fn test_cli_validator_usize_4() {
    match cli_validator_usize("42".to_string()) {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
}
