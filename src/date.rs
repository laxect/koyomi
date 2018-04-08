//! 日付
use chrono::{Datelike, NaiveDate, Weekday};

use self::KoyomiError::*;

#[derive(Debug)]
pub enum KoyomiError {
    InvalidDate(String),
}

#[derive(Debug)]
pub struct Date {
    date: NaiveDate,
}

impl Date {
    pub fn parse(fmt: &str) -> Result<Self, KoyomiError> {
        NaiveDate::parse_from_str(fmt, "%Y-%m-%d")
            .or(NaiveDate::parse_from_str(fmt, "%Y/%m/%d"))
            .map(|date| Date { date: date })
            .map_err(|_| InvalidDate(fmt.into()))
    }

    pub fn year(&self) -> i32 {
        self.date.year()
    }

    pub fn month(&self) -> u32 {
        self.date.month()
    }

    pub fn day(&self) -> u32 {
        self.date.day()
    }

    pub fn weekday(&self) -> String {
        let weekday = match self.date.weekday() {
            Weekday::Mon => "月",
            Weekday::Tue => "火",
            Weekday::Wed => "水",
            Weekday::Thu => "木",
            Weekday::Fri => "金",
            Weekday::Sat => "土",
            Weekday::Sun => "日",
        };

        weekday.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_hyphen_format() {
        assert!(Date::parse("2018-01-01").is_ok());
    }

    #[test]
    fn parse_with_slash_format() {
        assert!(Date::parse("2018/01/01").is_ok());
    }

    #[test]
    fn parse_with_invalid_format() {
        assert!(Date::parse("2018 01 01").is_err());
    }

    #[test]
    fn year_of_date() {
        let date = Date::parse("2018-12-24").unwrap();
        assert_eq!(date.year(), 2018);
    }

    #[test]
    fn month_of_date() {
        let date = Date::parse("2018-12-24").unwrap();
        assert_eq!(date.month(), 12);
    }

    #[test]
    fn day_of_date() {
        let date = Date::parse("2018-12-24").unwrap();
        assert_eq!(date.day(), 24);
    }

    #[test]
    fn sunday_of_date() {
        let date = Date::parse("2018-04-01").unwrap();
        assert_eq!(date.weekday(), "日");
    }

    #[test]
    fn monday_of_date() {
        let date = Date::parse("2018-04-02").unwrap();
        assert_eq!(date.weekday(), "月");
    }

    #[test]
    fn tuesday_of_date() {
        let date = Date::parse("2018-04-03").unwrap();
        assert_eq!(date.weekday(), "火");
    }

    #[test]
    fn wednesday_of_date() {
        let date = Date::parse("2018-04-04").unwrap();
        assert_eq!(date.weekday(), "水");
    }

    #[test]
    fn thursday_of_date() {
        let date = Date::parse("2018-04-05").unwrap();
        assert_eq!(date.weekday(), "木");
    }

    #[test]
    fn friday_of_date() {
        let date = Date::parse("2018-04-06").unwrap();
        assert_eq!(date.weekday(), "金");
    }

    #[test]
    fn saturday_of_date() {
        let date = Date::parse("2018-04-07").unwrap();
        assert_eq!(date.weekday(), "土");
    }
}
