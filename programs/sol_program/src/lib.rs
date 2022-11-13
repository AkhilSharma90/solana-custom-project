pub mod state;
pub mod ins;

use anchor_lang::prelude::*;
use ins::*;

declare_id!("EUFVQZe72N6zcPjYHE5KtqPgU8Z6pv5u3YwshaMKM2SL");

#[program]
pub mod sol_program {
    use super::*;
    pub fn init_data(ctx: Context<InitMyData>) -> ProgramResult {
        
        let acc = &mut ctx.accounts.data;
        acc.number = 0; 
        acc.message = String::from("MyData initialized!");

        // we store the public key of the signer to the owner field
        // of MyData
        acc.owner = ctx.accounts.owner.key();
                           
        Ok(())
    }
  
    pub fn update_data (ctx : Context<UpdateMyData>, number : u8, message : String) -> ProgramResult {

        let acc = &mut ctx.accounts.data;
        acc.number = number;
        acc.message = message;
  
        Ok(())
    }
}