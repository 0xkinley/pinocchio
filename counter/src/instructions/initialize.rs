use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;
use crate::{
    error::CounterError,
    state::{
        utils::{load_ix_data, DataLen},
        CounterState
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct InitializeCounterIxData {
    pub owner: Pubkey
}

impl DataLen for InitializeCounterIxData {
    const LEN: usize = core::mem::size_of::<InitializeCounterIxData>();
}

pub fn process_initialize_counter(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        owner_acc, 
        counter_state_acc, 
        sysvar_rent_acc, 
        _system_program, 
        _rest @ ..
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !owner_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !counter_state_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<InitializeCounterIxData>(data)? };

    if ix_data.owner.ne(owner_acc.key()) {
        return Err(CounterError::InvalidOwner.into());
    }


    let seeds = &[CounterState::SEED.as_bytes(), &ix_data.owner];

    let (derived_my_state_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_my_state_pda.ne(counter_state_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    let signer_seeds = [
        Seed::from(CounterState::SEED.as_bytes()),
        Seed::from(&ix_data.owner),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: owner_acc,
        to: counter_state_acc,
        space: CounterState::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(CounterState::LEN),
    }
    .invoke_signed(&signers)?;

    CounterState::initialize(counter_state_acc, ix_data, bump)?;

    Ok(())
}