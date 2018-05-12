extern crate chrono;

mod calendar;
mod date;
mod era;
mod holiday;

pub use date::Date;

pub type KoyomiResult<T> = Result<T, KoyomiError>;

#[derive(Debug)]
pub enum KoyomiError {
    InvalidFormat(String),
    InvalidTerm(Date, Date),
    NotEnough,
    NoTomorrow(i32, u32, u32),
    NoYesterday(i32, u32, u32),
}
