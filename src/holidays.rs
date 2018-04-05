//! # 国民の祝日定義
//!
//! https://ja.wikipedia.org/wiki/国民の祝日
const HOLIDAYS: [(&str, u8, u8, u16, Option<u16>); 16] = [
    // 名称, 月, 日, 制定年, 終了年
    ("元日", 1, 1, 1948, None),
    ("成人の日", 1, 15, 1948, Some(1999)),
    ("建国記念日", 2, 11, 1967, None),
    ("天皇誕生日", 4, 29, 1948, Some(1988)),
    ("みどりの日", 4, 29, 1989, Some(2006)),
    ("昭和の日", 4, 29, 2007, None),
    ("憲法記念日", 5, 3, 1948, None),
    ("みどりの日", 5, 4, 2007, None),
    ("こどもの日", 5, 5, 1948, None),
    ("海の日", 7, 20, 1996, Some(2002)),
    ("山の日", 8, 11, 2016, None),
    ("敬老の日", 9, 15, 1966, Some(2002)),
    ("体育の日", 10, 10, 1966, Some(1999)),
    ("文化の日", 11, 3, 1948, None),
    ("勤労感謝の日", 11, 23, 1948, None),
    ("天皇誕生日", 12, 23, 1989, Some(2019)),
];
