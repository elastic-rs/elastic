use nom::simple_errors::Err as NomError;
use std::str;

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
    Escaped(&'a str),
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

/* Parse `yyyy` as a 4 digit year. */
named!(year(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('y'), 4) >>
        (DateFormatToken::Year)
    )
);

/* Parse `MM` as a 2 digit month of year. */
named!(month(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('M'), 2) >>
        (DateFormatToken::Month)
    )
);

/* Parse `dd` as a 2 digit day of month. */
named!(day_of_month(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('d'), 2) >>
        (DateFormatToken::DayOfMonth)
    )
);

/* Parse `DDD` as a 3 digit day of year. */
named!(day_of_year(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('D'), 3) >>
        (DateFormatToken::DayOfYear)
    )
);

/* Parse `HH` as a 2 digit hour of day (24hr). */
named!(hour(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('H'), 2) >>
        (DateFormatToken::Hour)
    )
);

/* Parse `mm` as a 2 digit minute of hour. */
named!(minute(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('m'), 2) >>
        (DateFormatToken::Minute)
    )
);

/* Parse `ss` as a 2 digit second of minute. */
named!(second(&[u8]) -> DateFormatToken,
    do_parse!(
        count!(char!('s'), 2) >>
        (DateFormatToken::Second)
    )
);

/* Parse `.SSS` as a 3 digit millisecond of second. */
named!(millisecond(&[u8]) -> DateFormatToken,
    do_parse!(
        tag!(".") >>
        count!(char!('S'), 3) >>
        (DateFormatToken::Millisecond)
    )
);

/* Parse `Z` as a Utc timezone. */
named!(utc(&[u8]) -> DateFormatToken,
    do_parse!(
        char!('Z') >>
        (DateFormatToken::Utc)
    )
);

fn is_delim(i: u8) -> bool {
    i == b'-' || i == b':' || i == b'.' || i == b'/' || i == b' '
}

/* Parse a stream of `.`, `-`, `/`, `:` or ` ` as delimiters. */
named!(delim(&[u8]) -> DateFormatToken,
    do_parse!(
        i: take_while1!(is_delim) >>
        (DateFormatToken::Delim(str::from_utf8(i).unwrap()))
    )
);

/* Parse a stream of characters between `'`. */
named!(escaped(&[u8]) -> DateFormatToken,
    do_parse!(
        i: delimited!(char!('\''), is_not!("'"), char!('\'')) >>
        (DateFormatToken::Escaped(str::from_utf8(i).unwrap()))
    )
);

#[cfg(test)]
mod tests {
    use self::DateFormatToken::*;
    use super::*;

    fn assert_parse(i: &[u8], expected: Vec<DateFormatToken>) {
        let (_, result) = format(i).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_basic_date() {
        assert_parse(b"yyyyMMdd", vec![Year, Month, DayOfMonth]);
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
                Utc,
            ],
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
                Utc,
            ],
        );
    }

    #[test]
    fn parse_basic_ordinal_date() {
        assert_parse(b"yyyyDDD", vec![Year, DayOfYear]);
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
                Millisecond,
            ],
        );
    }
}
