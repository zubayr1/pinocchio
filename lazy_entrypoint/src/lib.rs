use pinocchio::{
    default_allocator, default_panic_handler, entrypoint::InstructionContext,
    lazy_program_entrypoint, msg, ProgramResult,
};

lazy_program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    msg!("Hello from my lazy program!");
    Ok(())
}
