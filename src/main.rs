use chrono::{prelude::*, Days, Duration, Months};
use num_traits::FromPrimitive;
use owo_colors::{
    colors::{Black, Default, White},
    OwoColorize,
};

pub fn calendar() -> String {
    let now = chrono::offset::Local::now();

    let mut output = String::new();

    // Month label
    output += &format!(
        "{: ^21}\n",
        format!(
            "{} {}",
            Month::from_u32(now.month()).unwrap().name(),
            now.year(),
        )
    );

    // Week labels, starting from sunday
    for d in [6, 0, 1, 2, 3, 4, 5] {
        let weekday = Weekday::from_u32(d).unwrap();
        let day_str = &weekday.to_string()[0..2];
        output += &format!("{} ", day_str);
    }
    output.push_str("\n");

    let mut curr = now.with_day(1).unwrap();

    // Skip days before the first of the month
    for _ in 1..curr.weekday().number_from_sunday() {
        output += "   ";
    }

    // Add days
    while curr.month() == now.month() {
        output += &if curr.day() == now.day() {
            format!("{: >2}", curr.day())
                .fg::<Black>()
                .bg::<White>()
                .to_string()
        } else {
            format!("{: >2}", curr.day())
                .fg::<White>()
                .bg::<Default>()
                .to_string()
        };
        output += " ";
        if curr.weekday().number_from_sunday() == 7 {
            output += "\n";
        }
        curr = curr.checked_add_signed(Duration::days(1)).unwrap();
    }
    output
}

fn main() {
    let cal = calendar();
}
