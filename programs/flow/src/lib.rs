use {
    anchor_lang::prelude::*,
    
    crate::instructions::*
};

mod collections;
mod instructions;
mod error;


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
}
