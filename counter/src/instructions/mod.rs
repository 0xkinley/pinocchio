use pinocchio::program_error::ProgramError;

pub mod increment;
pub mod decrement;
pub mod initialize;

pub use increment::*;
pub use decrement::*;
pub use initialize::*;

#[repr(u8)]
pub enum CounterInstruction{ 
    Initialize,   
    Increment,
    Decrement,
    
}

impl TryFrom<&u8> for CounterInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CounterInstruction::Initialize),
            1 => Ok(CounterInstruction::Increment),
            2 => Ok(CounterInstruction::Decrement),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}