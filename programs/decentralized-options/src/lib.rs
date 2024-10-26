use anchor_lang::prelude::*;

declare_id!("G8b6rYrFiyT87fGag8ghq2MUuhRepJ6mDDdwDPEftcSh");

#[program]
pub mod options {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let option_counter = &mut ctx.accounts.option_counter;
        option_counter.count = 0;
        Ok(())
    }

    pub fn list_option(
        ctx: Context<ListOption>,
        strike_price: u64,
        expiry_date_and_time: i64,
        is_call: bool,
        option_price: u64
    ) -> Result<()> {
        // Incriment the number of options (used in the PDA seed for new option)
        let option_counter = &mut ctx.accounts.option_counter;
        option_counter.count = option_counter.count.checked_add(1).unwrap();

        // Create new option
        let option_data = &mut ctx.accounts.option_data;
        option_data.seller = ctx.accounts.seller.key();
        option_data.strike_price = strike_price;
        option_data.expiry_date_and_time = expiry_date_and_time;
        option_data.is_call = is_call;
        option_data.option_price = option_price;

        // Probably want to get it to return the seed or some identifier for the front end
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, 
        payer=user, 
        space=DISCRIMINATOR + OptionCounter::INIT_SPACE
    )]
    pub option_counter: Account<'info, OptionCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ListOption<'info> {
    #[account(init, 
        payer=seller, 
        space=DISCRIMINATOR + OptionData::INIT_SPACE
    )]
    pub option_data: Account<'info, OptionData>,
    #[account(mut)]
    pub option_counter: Account<'info, OptionCounter>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct OptionCounter {
    pub count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct OptionData {
    //pub asset_price_feed: Pubkey,
    pub seller: Pubkey,
    pub buyer: Option<Pubkey>,
    pub strike_price: u64,
    pub expiry_date_and_time: i64,
    pub is_call: bool,
    pub option_price: u64,
}
 
const DISCRIMINATOR: usize = 8;