use thiserror::Error;

#[derive(Error, Debug)]
pub enum RangeCoderError {
    #[error("Overflow happend while lower_bound uppdating {lower_bound} + {add_val}")]
    LowerBoundOverflow { lower_bound: u64, add_val: u64 },
}
