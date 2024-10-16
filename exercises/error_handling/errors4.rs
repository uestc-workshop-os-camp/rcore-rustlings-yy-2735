// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value < 0 {
            return Err(CreationError::Negative);
        } else if value ==0 {
            return Err(CreationError::Zero);
        }

        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        PositiveNonzeroInteger::new(-10),
        Err(CreationError::Negative)
        
    );
    //
    assert_eq!(
        PositiveNonzeroInteger::new(0),
        Err(CreationError::Zero)
    );
}
