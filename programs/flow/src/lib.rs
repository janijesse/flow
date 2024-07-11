use {
    anchor_lang::prelude::*,
    
    crate::instructions::*
};

mod collections;
mod instructions;
mod error;
mod utils;


declare_id!("7Vw3DhQ98R5ke8WxARHWmNmZT11zg3AZ8LxuNEmGkrB7");

#[program]
pub mod flow {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        nombre: String,
        precio_ticket: u64
    ) -> Result<()> {
        instructions::create_event::handler(ctx, nombre, precio_ticket)
    }

     pub fn sponsor_event (ctx: Context<Sponsor>, cantidad: u64) -> Result<()> {
        instructions::sponsor::handler(ctx, cantidad)
    }

    pub fn buy_tickets (ctx: Context<BuyTickets>, cantidad: u64) -> Result<()> {
        instructions::buy_tickets::handler(ctx, cantidad)
    }

     pub fn withdraw_funds(ctx: Context<WithdrawFunds>, cantidad: u64) -> Result<()> {
        instructions::withdraw_funds::handle(ctx, cantidad)
    }

    pub fn close_event (ctx: Context<CloseEvent>) -> Result<()> {
        instructions::close_event::handler(ctx)
    }

    pub fn withdraw_earnings(ctx: Context<WithdrawEarnings>) -> Result<()> {
        instructions::withdraw_earnings::handler(ctx)
    }
}
