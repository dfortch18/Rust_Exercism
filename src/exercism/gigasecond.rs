use time::PrimitiveDateTime as DateTime;

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(time::Duration::seconds(GIGASECOND))
}
