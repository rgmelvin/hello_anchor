use anchor_lang::prelude::*;

declare_id!("2oy6dWTL4PB4JTBWbfwRphjQjYXnSx7CLpicAXJCX2pV");

#[program]
mod hello_world {
    use super::*;
    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!(
            "Hello World!"
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}