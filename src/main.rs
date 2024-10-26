use std::error::Error;
use std::num::ParseFloatError;
use std::str::FromStr;
use time::{Date, Month, OffsetDateTime, UtcOffset};

// 0-0:1.0.0(241025191816S)
//
// 1-0:1.8.1(002654.919*kWh)
//
// 1-0:1.8.2(002420.293*kWh)
//
// 1-0:2.8.1(006254.732*kWh)
//
// 1-0:2.8.2(002457.202*kWh)

// line.strip_prefix("1-0:1.8.1)
fn strip_prefix_and_suffix<'a>(line: &'a str, prefix: &str, suffix: &str) -> Option<&'a str> {
    if line.starts_with(prefix) && line.ends_with(suffix) {
        Some(&line[prefix.len()..line.len() - suffix.len()])
    } else {
        None
    }
}

fn parse_kwh(line: &str, prefix: &str) -> Result<Option<f64>, ParseFloatError> {
    match strip_prefix_and_suffix(line, prefix, "*kWh)") {
        Some(kwh) => f64::from_str(kwh).map(Some),
        None => Ok(None),
    }
}

fn parse_date_time(line: &str) -> Result<Option<OffsetDateTime>, Box<dyn Error>> {
    const DATA_LEN: usize = 13;
    match strip_prefix_and_suffix(line, "0-0:1.0.0(", ")") {
        Some(yymmddhhmmssx) => {
            if yymmddhhmmssx.len() == DATA_LEN
                && yymmddhhmmssx
                    .chars()
                    .nth(DATA_LEN - 1)
                    .map(|summer_or_winter| summer_or_winter == 'S' || summer_or_winter == 'W')
                    .unwrap_or(false)
            {
                let yy = i32::from_str(&yymmddhhmmssx[0..2])?;
                let mm = Month::try_from(u8::from_str(&yymmddhhmmssx[2..4])?)?;
                let dd = u8::from_str(&yymmddhhmmssx[4..6])?;
                let hours = u8::from_str(&yymmddhhmmssx[6..8])?;
                let mins = u8::from_str(&yymmddhhmmssx[8..10])?;
                let secs = u8::from_str(&yymmddhhmmssx[10..12])?;
                let date = Date::from_calendar_date(2000 + yy, mm, dd)?;
                let datetime = date.with_hms(hours, mins, secs)?;
                let datetime = datetime.assume_offset(UtcOffset::UTC);
                Ok(Some(datetime))
            } else {
                Ok(None) // I should (but am not going to) define an error type here
            }
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_prefix_and_suffix_prefix_mismatch_expect_none() {
        assert_eq!(
            strip_prefix_and_suffix("1-0:2.8.2(002457.202*kWh)", "mismatch", "*kWh)"),
            None
        )
    }

    #[test]
    fn strip_prefix_and_suffix_suffix_mismatch_expect_none() {
        assert_eq!(
            strip_prefix_and_suffix("1-0:2.8.2(002457.202*kWh)", "1-0:2.8.2(", "mismatch)"),
            None
        )
    }

    #[test]
    fn strip_prefix_and_suffix_match_expect_sub_string() {
        assert_eq!(
            strip_prefix_and_suffix("1-0:2.8.2(002457.202*kWh)", "1-0:2.8.2(", "*kWh)"),
            Some("002457.202")
        )
    }

    #[test]
    fn parse_kwh_expect_float() {
        assert_eq!(parse_kwh("prefix(12.34*kWh)", "prefix("), Ok(Some(12.34)))
    }

    #[test]
    fn parse_kwh_mismatch_expect_none() {
        assert_eq!(parse_kwh("prefix(12.34*kWh)", "mismatch"), Ok(None));
        assert_eq!(parse_kwh("prefix(12.34*mismatch)", "prefix("), Ok(None))
    }

    #[test]
    fn parse_kwh_bad_float_format_expect_err() {
        assert!(parse_kwh("prefix(bad-float*kWh)", "prefix(").is_err())
    }

    #[test]
    fn parse_date_time_mismatch_expect_none() {
        assert_eq!(parse_kwh("prefix(241025191816S)", "mismatch"), Ok(None));
        assert_eq!(
            parse_kwh("prefix(241025191816S)mismatch", "prefix("),
            Ok(None)
        )
    }

    #[test]
    fn parse_date_time_good_date_returned() {
        let datetime = parse_date_time("0-0:1.0.0(241025191816S)")
            .expect("Some(date) expected here")
            .expect("date expected here");
        assert_eq!(datetime.year(), 2024);
        assert_eq!(datetime.month(), Month::October);
        assert_eq!(datetime.day(), 25);
        assert_eq!(datetime.hour(), 19);
        assert_eq!(datetime.minute(), 18);
        assert_eq!(datetime.second(), 16);
        assert_eq!(datetime.offset(), UtcOffset::UTC);
    }

    #[test]
    fn parse_date_time_bad_date_error() {
        assert!(parse_date_time("0-0:1.0.0(249925191816S)").is_err());
        assert!(parse_date_time("0-0:1.0.0(240230191816S)").is_err());
        assert_eq!(
            parse_date_time("0-0:1.0.0(2410230191816S)").expect("No error expected here"),
            None
        );
        assert!(parse_date_time("0-0:1.0.0(241023241816S)").is_err());
        assert!(parse_date_time("0-0:1.0.0(241023196016S)").is_err());
        assert!(parse_date_time("0-0:1.0.0(241023195699S)").is_err());
        assert_eq!(
            parse_date_time("0-0:1.0.0(241023195609A)").expect("No error expected here"),
            None
        );
    }
}

fn main() {
    println!("Hello, world!");
    match strip_prefix_and_suffix("1-0:2.8.2(002457.202*kWh)", "1-0:1.8.2(", "*kWh)") {
        Some(x) => println!("?{}?", x),
        None => println!("OK"),
    }
    match strip_prefix_and_suffix("1-0:2.8.2(002457.202*kWh)", "1-0:2.8.2(", "*kWh)") {
        Some(x) => println!("OK ->{}<-", x),
        None => println!("?oh?"),
    }
    match parse_kwh("1-0:2.8.2(002457.202*kWh)", "1-0:2.8.2(") {
        Ok(Some(x)) => println!("OK ->{}<-", x),
        Ok(None) => println!("?None?"),
        Err(x) => println!("?Err({})?", x),
    }
    match parse_kwh("1-0:2.8.2(002457a202*kWh)", "1-0:2.8.2(") {
        Ok(Some(x)) => println!("?{}?", x),
        Ok(None) => println!("?None?"),
        Err(x) => println!("expected error: {}", x),
    }
    match parse_kwh("1-0:2.8.1(002457.202*kWh)", "1-0:2.8.2(") {
        Ok(Some(x)) => println!("?Ok(Some({}))?", x),
        Ok(None) => println!("ok: None"),
        Err(x) => println!("?Err({})?", x),
    }
    println!(
        "{}",
        parse_date_time("0-0:1.0.0(241025191816S)")
            .expect("Some(date) expected here")
            .expect("date expected here")
    );
}
