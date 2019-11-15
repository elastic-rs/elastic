use std::str;

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{
        alpha1,
        char,
    },
    combinator::map,
    error::ErrorKind as NomErrorKind,
    multi::{
        count,
        many1,
    },
    sequence::{
        delimited,
        preceded,
    },
    Err as NomErr,
    IResult,
};

pub type Error = NomErrorKind;

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
    match format(input.as_bytes()) {
        Ok(result) => Ok(result.1),
        Err(err) => {
            match err {
                NomErr::Error((_data, kind)) | NomErr::Failure((_data, kind)) => Err(kind),
                // TODO: how to handle `NomErr::Incomplete`?
                NomErr::Incomplete(_) => Err(NomErrorKind::Complete),
            }
        }
    }
}

fn format(input: &[u8]) -> IResult<&[u8], Vec<DateFormatToken>> {
    many1(alt((
        year,
        month,
        day_of_month,
        day_of_year,
        hour,
        minute,
        second,
        millisecond,
        utc,
        escaped,
        delim,
    )))(input)
}

/* Parse `yyyy` as a 4 digit year. */
fn year(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('y'), 4), |_| DateFormatToken::Year)(input)
}

/* Parse `MM` as a 2 digit month of year. */
fn month(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('M'), 2), |_| DateFormatToken::Month)(input)
}

/* Parse `dd` as a 2 digit day of month. */
fn day_of_month(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('d'), 2), |_| DateFormatToken::DayOfMonth)(input)
}

/* Parse `DDD` as a 3 digit day of year. */
fn day_of_year(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('D'), 3), |_| DateFormatToken::DayOfYear)(input)
}

/* Parse `HH` as a 2 digit hour of day (24hr). */
fn hour(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('H'), 2), |_| DateFormatToken::Hour)(input)
}

/* Parse `mm` as a 2 digit minute of hour. */
fn minute(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('m'), 2), |_| DateFormatToken::Minute)(input)
}

/* Parse `ss` as a 2 digit second of minute. */
fn second(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(count(char('s'), 2), |_| DateFormatToken::Second)(input)
}

/* Parse `.SSS` as a 3 digit millisecond of second. */
fn millisecond(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(preceded(char('.'), count(char('S'), 3)), |_| {
        DateFormatToken::Millisecond
    })(input)
}

/* Parse `Z` as a Utc timezone. */
fn utc(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(char('Z'), |_| DateFormatToken::Utc)(input)
}

fn is_delim(i: u8) -> bool {
    i == b'-' || i == b':' || i == b'.' || i == b'/' || i == b' '
}

/* Parse a stream of `.`, `-`, `/`, `:` or ` ` as delimiters. */
fn delim(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(take_while1(is_delim), |i| {
        DateFormatToken::Delim(str::from_utf8(i).unwrap())
    })(input)
}

/* Parse a stream of characters between `'`. */
fn escaped(input: &[u8]) -> IResult<&[u8], DateFormatToken> {
    map(delimited(char('\''), alpha1, char('\'')), |i| {
        DateFormatToken::Escaped(str::from_utf8(i).unwrap())
    })(input)
}

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
