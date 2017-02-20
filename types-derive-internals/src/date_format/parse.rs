use std::str;
use nom::simple_errors::Err as NomError;

pub type Error = NomError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateFormatToken<'a> {
    Year,
    Month,
    DayOfMonth,
    DayOfYear,
    Hour,
    Minute,
    Second,
    Millisecond,
    Utc,
    Delim(&'a str),
    Escaped(&'a str)
}

pub fn to_tokens<'a>(input: &'a str) -> Result<Vec<DateFormatToken<'a>>, Error> {
    format(input.as_bytes()).to_result()
}

named!(format(&[u8]) -> Vec<DateFormatToken>,
    complete!(tokens)
);

named!(tokens(&[u8]) -> Vec<DateFormatToken>,
    many1!(
        alt!(
            year |
            month |
            day_of_month |
            day_of_year |
            hour |
            minute |
            second |
            millisecond |
            utc |
            escaped |
            delim
        )
    )
);

/// Parse `yyyy` as a 4 digit year.
named!(year(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('y'), 4), 
        |r| DateFormatToken::Year
    )
);

/// Parse `MM` as a 2 digit month of year.
named!(month(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('M'), 2), 
        |r| DateFormatToken::Month
    )
);

/// Parse `dd` as a 2 digit day of month.
named!(day_of_month(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('d'), 2), 
        |r| DateFormatToken::DayOfMonth
    )
);

/// Parse `DDD` as a 3 digit day of year.
named!(day_of_year(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('D'), 3), 
        |r| DateFormatToken::DayOfYear
    )
);

/// Parse `HH` as a 2 digit hour of day (24hr).
named!(hour(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('H'), 2), 
        |r| DateFormatToken::Hour
    )
);

/// Parse `mm` as a 2 digit minute of hour.
named!(minute(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('m'), 2), 
        |r| DateFormatToken::Minute
    )
);

/// Parse `ss` as a 2 digit second of minute.
named!(second(&[u8]) -> DateFormatToken, 
    map!(
        count!(char!('s'), 2), 
        |r| DateFormatToken::Second
    )
);

/// Parse `.SSS` as a 3 digit millisecond of second.
named!(millisecond(&[u8]) -> DateFormatToken, 
    map!(
        do_parse!(
            tag!(".") >>
            count!(char!('S'), 3) >>
            ()
        ), 
        |r| DateFormatToken::Millisecond
    )
);

/// Parse `Z` as a UTC timezone
named!(utc(&[u8]) -> DateFormatToken, 
    map!(
        char!('Z'), 
        |r| DateFormatToken::Utc
    )
);

fn is_delim(i: u8) -> bool {
    i == b'-' || i == b':' || i == b'.' || i == b'/' || i == b' ' 
}

/// Parse a stream of `.`, `-`, `/`, `:` or ` ` as delimiters.
named!(delim(&[u8]) -> DateFormatToken, 
    map!(
        take_while1!(is_delim), 
        |r| DateFormatToken::Delim(str::from_utf8(r).unwrap())
    )
);

/// Parse a stream of characters between `'`.
named!(escaped(&[u8]) -> DateFormatToken, 
    map!(
        delimited!(char!('\''), is_not!("'"), char!('\'')), 
        |r| DateFormatToken::Escaped(str::from_utf8(r).unwrap())
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use self::DateFormatToken::*;

    fn assert_parse(i: &[u8], expected: Vec<DateFormatToken>) {
        let (_, result) = format(i).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_basic_date() {
        assert_parse(
            b"yyyyMMdd",
            vec![Year, Month, DayOfMonth]
        );
    }

    #[test]
    fn parse_basic_date_time() {
        assert_parse(
            b"yyyyMMdd'T'HHmmss.SSSZ", 
            vec![
                Year, 
                Month, 
                DayOfMonth, 
                Escaped("T"), 
                Hour, 
                Minute, 
                Second,
                Millisecond, 
                Utc
            ]
        );
    }

    #[test]
    fn parse_basic_date_time_no_millis() {
        assert_parse(
            b"yyyyMMdd'T'HHmmssZ", 
            vec![
                Year, 
                Month, 
                DayOfMonth, 
                Escaped("T"), 
                Hour, 
                Minute, 
                Second,
                Utc
            ]
        );
    }

    #[test]
    fn parse_basic_ordinal_date() {
        assert_parse(
            b"yyyyDDD", 
            vec![
                Year,
                DayOfYear
            ]
        );
    }

    #[test]
    fn parse_date_hour_minute_second_millis() {
        assert_parse(
            b"yyyy-MM-dd'T'HH:mm:ss.SSS", 
            vec![
                Year,
                Delim("-"), 
                Month, 
                Delim("-"),
                DayOfMonth, 
                Escaped("T"), 
                Hour,
                Delim(":"),
                Minute, 
                Delim(":"),
                Second,
                Millisecond
            ]
        );
    }
}