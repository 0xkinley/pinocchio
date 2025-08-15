use super::utils::{DataLen, Initialized};

use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use bytemuck::Zeroable;

use crate::{
    error::CounterError,
    instructions::InitializeCounterIxData,
    state::try_from_account_info_mut,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Zeroable, shank::ShankAccount)]
pub struct CounterState {    
    pub count: u64,
    pub owner: Pubkey,
    pub is_initialized: u8,    
    pub bump: u8,
}

impl DataLen for CounterState {
    const LEN: usize = core::mem::size_of::<CounterState>();
}

impl Initialized for CounterState {
    fn is_initialized(&self) -> bool {
        self.is_initialized > 0
    }
}

impl CounterState {
    pub const SEED: &'static str = "counter";


    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
        if derived != *pda {
            return Err(CounterError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        counter_state_account: &AccountInfo,
        ix_data: &InitializeCounterIxData,
        bump: u8,
      ) -> ProgramResult{
        let counter_state = unsafe { try_from_account_info_mut::<CounterState>(counter_state_account) }?;
        counter_state.is_initialized = 1;
        counter_state.owner = ix_data.owner;
        counter_state.count = 0;
        counter_state.bump = bump;

        Ok(())
    }

    pub fn increment(&mut self, increment_by: u64) -> ProgramResult {
        self.count = self.count
            .checked_add(increment_by)
            .ok_or(CounterError::Overflow)?;
        Ok(())
    }

    pub fn decrement(&mut self, decrement_by: u64) -> ProgramResult {
        self.count = self.count
            .checked_sub(decrement_by)
            .ok_or(CounterError::Underflow)?;
        Ok(())
    }

}