use anchor_lang::prelude::*;

declare_id!("53hgft52DHUKMPHGu1kusuwxFGk2T8qngwSw2SyGRNrX");

#[program]
pub mod day_3 {
    use super::*;
    use::std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>, key:String, value:String) -> Result<()> {
        let mut my_map = HashMap::new();

        // Add a key-value pair
        my_map.insert(key.to_string, value.to_string);
        msg!("{:?}", my_map[&key]);
        ok(());
    }

}

#[derive(Accounts)]
pub struct Initialize {}
