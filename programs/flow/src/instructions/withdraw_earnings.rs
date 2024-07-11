use {
    crate::collections::Event,
    anchor_lang::prelude::*,
    anchor_spl::token::*,
    crate::utils::*,
};

#[derive(Accounts)]
pub struct WithdrawEarnings<'info> {
    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(),
            organizador.key().as_ref(),
        ],
        bump = evento.bump_evento,
    )]
    pub evento: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_FLOW_TOKEN.as_ref(),
            evento.key().as_ref(), 
        ],
        bump = evento.bump_flow_token,
    )]
    pub flow_token: Box<Account<'info, Mint>>, 
    #[account(mut)]
    pub user_accepted_mint_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    
    pub user_event_mint_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_BOVEDA_GANANCIAS.as_ref(),
            evento.key().as_ref(), 
        ],
        bump = evento.bump_boveda_ganancias,
    )]
    pub boveda_ganancias: Box<Account<'info, TokenAccount>>, 
    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawEarnings>) -> Result<()> {
    let total_tokens = ctx.accounts.evento.sponsors;
    let tokens_to_burn = ctx.accounts.user_event_mint_ata.amount;
    let total_earnings = ctx.accounts.boveda_ganancias.amount;

    let share = calculate_share(total_tokens, tokens_to_burn);
    // calculate total earning amount based on share %
    let total_to_deposit = calculate_earnings(total_earnings, share);

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.flow_token.to_account_info(), 
                from: ctx.accounts.user_event_mint_ata.to_account_info(), 
                authority: ctx.accounts.organizador.to_account_info(), 
            },
        ),
        , 
    )?;

    let seeds = [
        Event::SEED_EVENTO.as_bytes(), 
        ctx.accounts.evento.organizador.as_ref(),
    ];
    let signer = &[&seeds[..]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.boveda_ganancias.to_account_info(),
                to: ctx.accounts.user_accepted_mint_ata.to_account_info(),
                authority: ctx.accounts.evento.to_account_info(),
            },
            signer,
        ),
        total_to_deposit,
    )?;

    Ok(())
}
