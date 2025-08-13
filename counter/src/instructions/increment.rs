use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey:: Pubkey,
    ProgramResult,
};

use crate::state::try_from_account_info_mut;

use crate::{
    error::CounterError,
    state::{
        utils::{load_ix_data, DataLen},
        CounterState
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct IncrementIxData {
    pub owner: Pubkey,
    pub increment_by: u64,
}

impl DataLen for IncrementIxData {
    const LEN: usize = core::mem::size_of::<IncrementIxData>();
}

pub fn process_increment_counter(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [owner_acc, counter_state_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !owner_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ix_data = unsafe { load_ix_data::<IncrementIxData>(data)? };

    if ix_data.owner.ne(owner_acc.key()) {
        return Err(CounterError::InvalidOwner.into());
    }

    let counter_state = unsafe { try_from_account_info_mut::<CounterState>(counter_state_acc) }?;
    counter_state.increment(ix_data.increment_by)?;

    Ok(())
}