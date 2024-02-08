//use solana-program crate and bring our needed items into the local namespace:
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

//every Solana program must define an entrypoint that tells the Solana runtime where to start executing your on chain code. Your program's entrypoint 
//should provide a public function named process_instruction:

//declare and export the program's entrypoint
entrypoint!(process_instruction);

//program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    //log a message to the Chain
    msg!("Hello.. Globe?");

    //gracefully exits the program
    Ok(())
}

//every on chain program should return the Ok result enum with a value of ().