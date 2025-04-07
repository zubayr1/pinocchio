use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::AddressInfo;

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_info: AccountInfo,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account_info_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (account_info.try_to_vec()?).len();

    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            &signer.key,
            &account_info_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            signer.clone(),
            account_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    address_info.serialize(&mut &mut account_info_account.data.borrow_mut()[..])?;

    Ok(())
}
