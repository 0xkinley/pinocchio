#![allow(unexpected_cfgs)]

use crate::instructions::{self, CounterInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CounterInstruction::try_from(ix_disc)? {
        CounterInstruction::Initialize => {
            msg!("Ix:0");
            instructions::process_initialize_counter(accounts, instruction_data)?;
            Ok(())

        }
        CounterInstruction::Increment => {
            msg!("Ix:1");
            instructions::process_increment_counter(accounts, instruction_data)?;
            Ok(())
        }
        CounterInstruction::Decrement => {
            msg!("Ix:2");
            instructions::process_decrement_counter(accounts, instruction_data)?;
            Ok(())
        }
    }
}
