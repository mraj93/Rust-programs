pub fn initialize(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
    // Defining a struct in Solana
    struct Person {
        my_name: String,
        my_age: u64,
    }

    let mut person1: Person = Person {
        my_name: name,
        my_age: age,
    };

    msg!("{} is {} years old", person1.my_name, person1.my_age);

    // Accessing and modifying struct fields
    person1.my_name = "Bob".to_string();
    person1.my_age = 18;

    msg!("{} is {} years old", person1.my_name, person1.my_age);

    Ok(())
}