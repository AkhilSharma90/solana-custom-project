use anchor_lang::prelude::*;
use crate::state::MyData;

#[derive(Accounts)]
pub struct InitMyData<'info> {
    
    #[account(init, payer = owner, space = 8 + 1 + 50 + 32)]
    pub data : Account<'info, MyData>,
    #[account(mut)]
    pub owner : Signer<'info>,

    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMyData<'info> {

   #[account(mut,has_one=owner)]
   pub data : Account<'info, MyData>,

   pub owner : Signer<'info>,
}