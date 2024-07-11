use {
    anchor_lang::prelude::*,
    anchor_spl::token::*,
    
    crate::utils::*,
    crate::collections::Event,
};

#[derive(Accounts)]
pub struct WithdrawEarnings<'info> {

    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(),
            evento.organizador.key().as_ref(),
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
    pub ata_moneda_pago: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub ata_flow_token: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_BOVEDA_GANANCIAS.as_ref(),
            evento.key().as_ref(), 
        ],
        bump = evento.bump_boveda_ganancias,
    )]
    pub boveda_ganancias: Box<Account<'info, TokenAccount>>, 
}

pub fn handler(ctx: Context<WithdrawEarnings>) -> Result<()> {
    
    let flow_token_vendidos = ctx.accounts.evento.sponsors; // en la tesoreria
    let flow_tokens = ctx.accounts.ata_flow_token.amount;
    let ganancias_totales = ctx.accounts.boveda_ganancias.amount;
    let share = calculate_share(flow_token_vendidos, flow_tokens);
    let ganancias = calculate_earnings(ganancias_totales, share);

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.flow_token.to_account_info(), 
                from: ctx.accounts.ata_flow_token.to_account_info(), 
                authority: ctx.accounts.organizador.to_account_info(), 
            },
        ),
        ctx.accounts.ata_flow_token.amount,
    )?;

    let seeds = [
        Event::SEED_EVENTO.as_bytes(), 
        ctx.accounts.evento.organizador.as_ref(),
        &[ctx.accounts.evento.bump_evento]
    ];
    
    let signer = &[&seeds[..]];
    
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.boveda_ganancias.to_account_info(),
                to: ctx.accounts.ata_moneda_pago.to_account_info(),
                authority: ctx.accounts.evento.to_account_info(),
            },
            signer,
        ),
        ganancias,
    )?;

    Ok(())
}
