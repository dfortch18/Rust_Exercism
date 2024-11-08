mod exercism;

fn print_separator() {
    println!("-----------------------------------------");
}

fn datetime(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(), 
        Time::from_hms(hour, minute, second).unwrap()
    )
}

fn main() {
    print_separator();

    println!("{}", exercism::hello::hello());

    print_separator();

    let reverse_string_inputs = vec!["robot", "Ramen", "I'm hungry!", "racecar", "drawer", "子猫", "Würstchenstand", "ผู้เขียนโปรแกรม"];

    for input in reverse_string_inputs {
        println!("Reverse of ({}): {}", input, exercism::reverse_string::reverse(input));
    }

    print_separator();

    let gigasecond_inputs = vec![
        datetime(2011, 4, 25, 0, 0, 0),
        datetime(1977, 6, 13, 0, 0, 0),
        datetime(1959, 7, 19, 0, 0, 0),
        datetime(2015, 1, 24, 22, 0, 0),
        datetime(2015, 1, 24, 23, 59, 59)
    ];

    for input in gigasecond_inputs {
        println!("Gigasecond after ({}): {}", input, exercism::gigasecond::after(input));
    }

    print_separator();
}
