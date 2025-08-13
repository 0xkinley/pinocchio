use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq, shank::ShankType)]
pub enum CounterError{
    InvalidInstructionData,
    PdaMismatch,
    InvalidOwner,
    Overflow,
    Underflow
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        Self::Custom(e as u32)
    }
}