use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

use crate::instructions::*;
use crate::state::AddressInfo;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Ok(account_info) = AddressInfo::try_from_slice(instruction_data) {
        return instructions::initialize::initialize(program_id, accounts, account_info);
    }

    Err(ProgramError::InvalidInstructionData)
}
