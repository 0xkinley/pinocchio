use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::{self, Pubkey},
    signer::Signer, system_program, sysvar::rent,
};

mod common;

#[test]
fn test_init_counter() {
    let (mut svm, fee_payer, program_id) = common::setup_svm_and_program();
    let fee_payer_pubkey = fee_payer.pubkey();

    let data = [
        vec![0],                                // Discriminator (1 byte) - Initialize
        0u64.to_le_bytes().to_vec(),            // count: u64 (8 bytes)
        fee_payer_pubkey.to_bytes().to_vec(),   // owner: Pubkey (32 bytes)
        0u8.to_le_bytes().to_vec(),            // is_initialized: u8 (1 byte)
        0u8.to_le_bytes().to_vec(),            // bump: u8 (1 byte)
        vec![0; 6],                           // padding: [u8; 6] (6 bytes)

    ]
    .concat();

    // counter Config PDA
    let seed = [(b"counter"), fee_payer_pubkey.as_ref()];
    let seeds = &seed[..];
    let (pda_counter, counter_bump) = Pubkey::find_program_address(seeds, &program_id);

    println!("pda_counter acc : {:?}", pda_counter);

    let instruction = vec![Instruction {
        program_id: program_id,
        accounts: vec![
            AccountMeta::new(fee_payer.pubkey(), true),
            AccountMeta::new(pda_counter, false),
            AccountMeta::new(rent::ID, false),
            AccountMeta::new(system_program::ID, false),      
        ],
        data
    }];

    let result = common::build_and_send_transaction(&mut svm, &fee_payer, instruction);

    println!("result: {:?}", result);

    assert!(result.is_ok());
}
