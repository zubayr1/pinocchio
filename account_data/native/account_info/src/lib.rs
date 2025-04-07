use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

mod instructions;
mod processor;
mod state;

entrypoint!(process_instruction);
