extern crate chrono;
extern crate koyomi;

use chrono::Datelike;

macro_rules! assert_holiday {
    ($y:expr, $m:expr, $d:expr, $expect:expr) => {
        let d = koyomi::Date::from_ymd($y, $m, $d).unwrap();
        assert_eq!(d.holiday().unwrap(), $expect);
    };

    ($name:expr, $days:expr) => {
        $days.iter().for_each(|&(y, m, d)| {
            assert_holiday!(y, m, d, $name);
        });
    };
}

fn years(from: i32) -> impl Iterator<Item = i32> {
    let current = chrono::Utc::now().year();

    (from..=current)
}

#[test]
fn new_years_day() {
    let name = "元日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 1, 1, name);
    });
}

#[test]
fn coming_of_age_day() {
    let name = "成人の日";

    (1948..=1999).for_each(|y| {
        assert_holiday!(y, 1, 15, name);
    });

    assert_holiday!(
        name,
        vec![
            (2000, 1, 10),
            (2001, 1, 8),
            (2002, 1, 14),
            (2003, 1, 13),
            (2004, 1, 12),
            (2005, 1, 10),
            (2006, 1, 9),
            (2007, 1, 8),
            (2008, 1, 14),
            (2009, 1, 12),
            (2010, 1, 11),
            (2011, 1, 10),
            (2012, 1, 9),
            (2013, 1, 14),
            (2014, 1, 13),
            (2015, 1, 12),
            (2016, 1, 11),
            (2017, 1, 9),
            (2018, 1, 8),
        ]
    );
}

#[test]
fn national_foundation_day() {
    let name = "建国記念日";

    years(1967).for_each(|y| {
        assert_holiday!(y, 2, 11, name);
    });
}

#[test]
fn vernal_equinox_day() {
    let name = "春分の日";

    assert_holiday!(
        name,
        vec![
            (1949, 3, 21),
            (1950, 3, 21),
            (1951, 3, 21),
            (1952, 3, 21),
            (1953, 3, 21),
            (1954, 3, 21),
            (1955, 3, 21),
            (1956, 3, 21),
            (1957, 3, 21),
            (1958, 3, 21),
            (1959, 3, 21),
            (1960, 3, 20),
            (1961, 3, 21),
            (1962, 3, 21),
            (1963, 3, 21),
            (1964, 3, 20),
            (1965, 3, 21),
            (1966, 3, 21),
            (1967, 3, 21),
            (1968, 3, 20),
            (1969, 3, 21),
            (1970, 3, 21),
            (1971, 3, 21),
            (1972, 3, 20),
            (1973, 3, 21),
            (1974, 3, 21),
            (1975, 3, 21),
            (1976, 3, 20),
            (1977, 3, 21),
            (1978, 3, 21),
            (1979, 3, 21),
            (1980, 3, 20),
            (1981, 3, 21),
            (1982, 3, 21),
            (1983, 3, 21),
            (1984, 3, 20),
            (1985, 3, 21),
            (1986, 3, 21),
            (1987, 3, 21),
            (1988, 3, 20),
            (1989, 3, 21),
            (1990, 3, 21),
            (1991, 3, 21),
            (1992, 3, 20),
            (1993, 3, 20),
            (1994, 3, 21),
            (1995, 3, 21),
            (1996, 3, 20),
            (1997, 3, 20),
            (1998, 3, 21),
            (1999, 3, 21),
            (2000, 3, 20),
            (2001, 3, 20),
            (2002, 3, 21),
            (2003, 3, 21),
            (2004, 3, 20),
            (2005, 3, 20),
            (2006, 3, 21),
            (2007, 3, 21),
            (2008, 3, 20),
            (2009, 3, 20),
            (2010, 3, 21),
            (2011, 3, 21),
            (2012, 3, 20),
            (2013, 3, 20),
            (2014, 3, 21),
            (2015, 3, 21),
            (2016, 3, 20),
            (2017, 3, 20),
            (2018, 3, 21),
        ]
    );
}

#[test]
fn birthday_of_showa_emperor() {
    let name = "天皇誕生日";

    (1948..=1988).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });
}

#[test]
fn green_day() {
    let name = "みどりの日";

    (1989..=2006).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });

    years(2007).for_each(|y| {
        assert_holiday!(y, 5, 4, name);
    });
}

#[test]
fn showa_day() {
    let name = "昭和の日";

    years(2007).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });
}

#[test]
fn constitution_day() {
    let name = "憲法記念日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 5, 3, name);
    });
}

#[test]
fn childrens_day() {
    let name = "こどもの日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 5, 5, name);
    });
}

#[test]
fn marine_day() {
    let name = "海の日";

    (1996..=2002).for_each(|y| {
        assert_holiday!(y, 7, 20, name);
    });

    assert_holiday!(
        name,
        vec![
            (2003, 7, 21),
            (2004, 7, 19),
            (2005, 7, 18),
            (2006, 7, 17),
            (2007, 7, 16),
            (2008, 7, 21),
            (2009, 7, 20),
            (2010, 7, 19),
            (2011, 7, 18),
            (2012, 7, 16),
            (2013, 7, 15),
            (2014, 7, 21),
            (2015, 7, 20),
            (2016, 7, 18),
            (2017, 7, 17),
            (2018, 7, 16),
        ]
    );
}

#[test]
fn mountain_day() {
    let name = "山の日";

    years(2016).for_each(|y| {
        assert_holiday!(y, 8, 11, name);
    });
}

#[test]
fn respect_for_the_aged_day() {
    let name = "敬老の日";

    (1966..=2002).for_each(|y| {
        assert_holiday!(y, 9, 15, name);
    });

    assert_holiday!(
        name,
        vec![
            (2003, 9, 15),
            (2004, 9, 20),
            (2005, 9, 19),
            (2006, 9, 18),
            (2007, 9, 17),
            (2008, 9, 15),
            (2009, 9, 21),
            (2010, 9, 20),
            (2011, 9, 19),
            (2012, 9, 17),
            (2013, 9, 16),
            (2014, 9, 15),
            (2015, 9, 21),
            (2016, 9, 19),
            (2017, 9, 18),
            (2018, 9, 17),
        ]
    );
}

#[test]
fn autumnal_equinox_holiday() {
    let name = "秋分の日";

    assert_holiday!(
        name,
        vec![
            (1948, 9, 23),
            (1949, 9, 23),
            (1950, 9, 23),
            (1951, 9, 24),
            (1952, 9, 23),
            (1953, 9, 23),
            (1954, 9, 23),
            (1955, 9, 24),
            (1956, 9, 23),
            (1957, 9, 23),
            (1958, 9, 23),
            (1959, 9, 24),
            (1960, 9, 23),
            (1961, 9, 23),
            (1962, 9, 23),
            (1963, 9, 24),
            (1964, 9, 23),
            (1965, 9, 23),
            (1966, 9, 23),
            (1967, 9, 24),
            (1968, 9, 23),
            (1969, 9, 23),
            (1970, 9, 23),
            (1971, 9, 24),
            (1972, 9, 23),
            (1973, 9, 23),
            (1974, 9, 23),
            (1975, 9, 24),
            (1976, 9, 23),
            (1977, 9, 23),
            (1978, 9, 23),
            (1979, 9, 24),
            (1980, 9, 23),
            (1981, 9, 23),
            (1982, 9, 23),
            (1983, 9, 23),
            (1984, 9, 23),
            (1985, 9, 23),
            (1986, 9, 23),
            (1987, 9, 23),
            (1988, 9, 23),
            (1989, 9, 23),
            (1990, 9, 23),
            (1991, 9, 23),
            (1992, 9, 23),
            (1993, 9, 23),
            (1994, 9, 23),
            (1995, 9, 23),
            (1996, 9, 23),
            (1997, 9, 23),
            (1998, 9, 23),
            (1999, 9, 23),
            (2000, 9, 23),
            (2001, 9, 23),
            (2002, 9, 23),
            (2003, 9, 23),
            (2004, 9, 23),
            (2005, 9, 23),
            (2006, 9, 23),
            (2007, 9, 23),
            (2008, 9, 23),
            (2009, 9, 23),
            (2010, 9, 23),
            (2011, 9, 23),
            (2012, 9, 22),
            (2013, 9, 23),
            (2014, 9, 23),
            (2015, 9, 23),
            (2016, 9, 22),
            (2017, 9, 23),
            (2018, 9, 23),
        ]
    );
}

#[test]
fn sports_day() {
    let name = "体育の日";

    (1966..=1999).for_each(|y| {
        assert_holiday!(y, 10, 10, name);
    });

    assert_holiday!(
        name,
        vec![
            (2000, 10, 9),
            (2001, 10, 8),
            (2002, 10, 14),
            (2003, 10, 13),
            (2004, 10, 11),
            (2005, 10, 10),
            (2006, 10, 9),
            (2007, 10, 8),
            (2008, 10, 13),
            (2009, 10, 12),
            (2010, 10, 11),
            (2011, 10, 10),
            (2012, 10, 8),
            (2013, 10, 14),
            (2014, 10, 13),
            (2015, 10, 12),
            (2016, 10, 10),
            (2017, 10, 9),
            (2018, 10, 8),
        ]
    );
}

#[test]
fn culture_day() {
    let name = "文化の日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 11, 3, name);
    });
}

#[test]
fn labor_thanksgiving_day() {
    let name = "勤労感謝の日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 11, 23, name);
    });
}

#[test]
fn birthday_of_heisei_emperor() {
    let name = "天皇誕生日";

    years(1989).for_each(|y| {
        assert_holiday!(y, 12, 23, name);
    });
}
