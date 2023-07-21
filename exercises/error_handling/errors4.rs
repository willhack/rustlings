// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            v if v < 0 => Err(CreationError::Negative),
            v if v == 0 => Err(CreationError::Zero),
            v => Ok(PositiveNonzeroInteger(v as u64))
        }
        // return if value < 0 {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Ok(PositiveNonzeroInteger(value as u64))
        // };
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
