use thiserror::Error;

#[derive(Error, Debug)]
pub enum RangeCoderError {
    #[error("Overflow happend while lower_bound uppdating {lower_bound} + {add_val} , {range}")]
    LowerBoundOverflow {
        lower_bound: u64,
        add_val: u64,
        range: u64,
    },
    #[error("Overflow happend when calc upper_bound {lower_bound} + {range}")]
    UpperBoundOverflow { lower_bound: u64, range: u64 },
}
