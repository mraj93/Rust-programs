pub fn get_day_of_the_week(
    _ctx: Context<Initialize>) -> Result<()> {

    let clock = Clock::get()?;
    let time_stamp = clock.unix_timestamp; // current timestamp

    let date_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let day_of_the_week = date_time.weekday();

    msg!("Week day is: {}", day_of_the_week);

    Ok(())
}
