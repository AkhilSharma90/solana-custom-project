use anchor_lang::prelude::*;

#[account]
pub struct MyData {
    pub number : u8,
    pub message : String, 
    pub owner : Pubkey,
}