pub fn initialize(ctx: Context<Initialize>,
                      a: u64,
                      b: u64) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        msg!("Hello Mukesh");
        Ok(())
}