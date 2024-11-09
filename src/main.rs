mod exercism;
use exercism::clock::Clock;

fn print_separator() {
    println!("-----------------------------------------");
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

fn main() {
    print_separator();

    println!("{}", exercism::hello::hello());

    print_separator();

    let reverse_string_inputs = vec![
        "robot",
        "Ramen",
        "I'm hungry!",
        "racecar",
        "drawer",
        "子猫",
        "Würstchenstand",
        "ผู้เขียนโปรแกรม",
    ];

    for input in reverse_string_inputs {
        println!(
            "Reverse of ({}): {}",
            input,
            exercism::reverse_string::reverse(input)
        );
    }

    print_separator();

    let gigasecond_inputs = vec![
        datetime(2011, 4, 25, 0, 0, 0),
        datetime(1977, 6, 13, 0, 0, 0),
        datetime(1959, 7, 19, 0, 0, 0),
        datetime(2015, 1, 24, 22, 0, 0),
        datetime(2015, 1, 24, 23, 59, 59),
    ];

    for input in gigasecond_inputs {
        println!(
            "Gigasecond after ({}): {}",
            input,
            exercism::gigasecond::after(input)
        );
    }

    print_separator();

    let clock_new_inputs = [
        [8, 0],
        [11, 9],
        [24, 0],
        [25, 0],
        [100, 0],
        [1, 60],
        [0, 160],
        [0, 1723],
        [25, 160],
        [201, 3001],
        [72, 8640],
        [-1, 15],
        [-25, 0],
        [-91, 0],
        [1, -40],
        [1, -160],
        [1, -4820],
        [2, -60],
        [-25, -160],
        [-121, -5810],
    ];

    let clock_add_minutes_clock_inputs = [
        Clock::new(10, 0),
        Clock::new(6, 41),
        Clock::new(0, 45),
        Clock::new(10, 0),
        Clock::new(0, 45),
        Clock::new(23, 59),
        Clock::new(5, 32),
        Clock::new(1, 1),
        Clock::new(10, 3),
        Clock::new(10, 3),
        Clock::new(10, 3),
        Clock::new(0, 3),
        Clock::new(0, 0),
        Clock::new(6, 15),
        Clock::new(5, 32),
        Clock::new(2, 20),
    ];
    let clock_add_minutes_minutes_inputs = [
        3, 0, 40, 61, 160, 2, 1500, 3500, -3, -30, -70, -4, -160, -160, -1500, -3000,
    ];

    let clock_equal_inputs = [
        [Clock::new(15, 37), Clock::new(15, 37)],
        [Clock::new(15, 36), Clock::new(15, 37)],
        [Clock::new(14, 37), Clock::new(15, 37)],
        [Clock::new(10, 37), Clock::new(34, 37)],
        [Clock::new(3, 11), Clock::new(99, 11)],
        [Clock::new(22, 40), Clock::new(-2, 40)],
        [Clock::new(17, 3), Clock::new(-31, 3)],
        [Clock::new(13, 49), Clock::new(-83, 49)],
        [Clock::new(0, 1), Clock::new(0, 1441)],
        [Clock::new(2, 2), Clock::new(2, 4322)],
        [Clock::new(2, 40), Clock::new(3, -20)],
        [Clock::new(4, 10), Clock::new(5, -1490)],
        [Clock::new(6, 15), Clock::new(6, -4305)],
        [Clock::new(7, 32), Clock::new(-12, -268)],
        [Clock::new(18, 7), Clock::new(-54, -11513)],
        [Clock::new(24, 0), Clock::new(0, 0)],
    ];

    for inputs in clock_new_inputs {
        let hours = inputs[0];
        let minutes = inputs[1];
        let clock = Clock::new(hours, minutes);
        println!(
            "Clock with (hours: {}, minutes: {}), {}",
            hours, minutes, clock
        );
    }

    for (clock, &minutes_add) in clock_add_minutes_clock_inputs
        .iter()
        .zip(&clock_add_minutes_minutes_inputs)
    {
        let result = clock.add_minutes(minutes_add);

        println!(
            "Clock ({}) with minutes added ({}): {}",
            clock, minutes_add, result
        );
    }

    for clocks in clock_equal_inputs {
        println!(
            "Clock ({}) equals other ({}): {}",
            clocks[0],
            clocks[1],
            clocks[0].eq(&clocks[1])
        );
    }

    print_separator();
}
