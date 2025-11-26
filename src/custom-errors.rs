use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum OperationError {
    DividedByZerrorError,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        match *self {
            OperationError::DividedByZerrorError => f.write_str("Cannot divide by zero"),
        }
    }
}

impl Error for OperationError {
    fn description(&self) -> &str {
        match *self {
             OperationError::DividedByZerrorError => "Cannot divide by zero",
        }
    }
}

fn divide(divided: u32, divisor: u32) -> Result<u32, OperationError> {
    if divisor == 0u32 {
        Err(OperationError::DividedByZerrorError)
    } else {
        Ok(divided / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);
    println!("{:?}", result1);

    let result2 = divide(100, 2);
    println!("{:?}", result2.unwrap());
}
