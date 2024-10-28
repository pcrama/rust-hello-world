use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::borrow::Borrow;
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

    #[test]
    fn parse_lines_nonsense_returns_ok_none() {
        assert_eq!(
            parse_lines("a\nb\nc".lines()).expect("Ok(None) expected here"),
            None,
        )
    }

    #[test]
    fn parse_lines_happy_path() {
        assert_eq!(
            parse_lines("\n0-0:1.0.0(241025000000S)\n\n1-0:1.8.1(002654.919*kWh)\n\n1-0:1.8.2(002420.293*kWh)\n\n1-0:2.8.1(006254.732*kWh)\n\n1-0:2.8.2(002457.202*kWh)".lines()).expect("Ok(some meas) expected here"),
            Some(CompleteP1Measurement { timestamp: Date::from_calendar_date(2024, Month::October, 25).unwrap().midnight().assume_utc(), peak_hour_consumption: 2654.919, off_hour_consumption: 2420.293, peak_hour_injection: 6254.732, off_hour_injection: 2457.202 }),
        )
    }
}

#[derive(PartialEq, Debug)]
struct PartialP1Measurement {
    timestamp: Option<OffsetDateTime>,
    peak_hour_consumption: Option<f64>,
    off_hour_consumption: Option<f64>,
    peak_hour_injection: Option<f64>,
    off_hour_injection: Option<f64>,
}

#[derive(PartialEq, Debug)]
struct CompleteP1Measurement {
    timestamp: OffsetDateTime,
    peak_hour_consumption: f64,
    off_hour_consumption: f64,
    peak_hour_injection: f64,
    off_hour_injection: f64,
}

fn complete_p1_measurement(
    partial: PartialP1Measurement,
) -> Result<CompleteP1Measurement, PartialP1Measurement> {
    match partial {
        PartialP1Measurement {
            timestamp: Some(timestamp),
            peak_hour_consumption: Some(peak_hour_consumption),
            off_hour_consumption: Some(off_hour_consumption),
            peak_hour_injection: Some(peak_hour_injection),
            off_hour_injection: Some(off_hour_injection),
        } => Ok(CompleteP1Measurement {
            timestamp: timestamp,
            peak_hour_consumption: peak_hour_consumption,
            off_hour_consumption: off_hour_consumption,
            peak_hour_injection: peak_hour_injection,
            off_hour_injection: off_hour_injection,
        }),
        _ => Err(partial),
    }
}

fn step_partial_p1_measurement(
    partial: PartialP1Measurement,
    line: &str,
) -> Result<PartialP1Measurement, Box<dyn Error>> {
    match partial {
        PartialP1Measurement {
            timestamp: None, ..
        } => match parse_date_time(line)? {
            Some(timestamp) => Ok(PartialP1Measurement {
                timestamp: Some(timestamp),
                peak_hour_consumption: None,
                off_hour_consumption: None,
                peak_hour_injection: None,
                off_hour_injection: None,
            }),
            _ => Ok(partial),
        },
        PartialP1Measurement {
            timestamp: Some(timestamp),
            peak_hour_consumption: None,
            ..
        } => match parse_kwh(line, "1-0:1.8.1(")? {
            Some(kwh) => Ok(PartialP1Measurement {
                timestamp: Some(timestamp),
                peak_hour_consumption: Some(kwh),
                off_hour_consumption: None,
                peak_hour_injection: None,
                off_hour_injection: None,
            }),
            _ => Ok(partial),
        },
        PartialP1Measurement {
            timestamp: Some(timestamp),
            peak_hour_consumption: Some(peak_hour_consumption),
            off_hour_consumption: None,
            ..
        } => match parse_kwh(line, "1-0:1.8.2(")? {
            Some(kwh) => Ok(PartialP1Measurement {
                timestamp: Some(timestamp),
                peak_hour_consumption: Some(peak_hour_consumption),
                off_hour_consumption: Some(kwh),
                peak_hour_injection: None,
                off_hour_injection: None,
            }),
            _ => Ok(partial),
        },
        PartialP1Measurement {
            timestamp: Some(timestamp),
            peak_hour_consumption: Some(peak_hour_consumption),
            off_hour_consumption: Some(off_hour_consumption),
            peak_hour_injection: None,
            ..
        } => match parse_kwh(line, "1-0:2.8.1(")? {
            Some(kwh) => Ok(PartialP1Measurement {
                timestamp: Some(timestamp),
                peak_hour_consumption: Some(peak_hour_consumption),
                off_hour_consumption: Some(off_hour_consumption),
                peak_hour_injection: Some(kwh),
                off_hour_injection: None,
            }),
            _ => Ok(partial),
        },
        PartialP1Measurement {
            timestamp: Some(timestamp),
            peak_hour_consumption: Some(peak_hour_consumption),
            off_hour_consumption: Some(off_hour_consumption),
            peak_hour_injection: Some(peak_hour_injection),
            off_hour_injection: None,
            ..
        } => match parse_kwh(line, "1-0:2.8.2(")? {
            Some(kwh) => Ok(PartialP1Measurement {
                timestamp: Some(timestamp),
                peak_hour_consumption: Some(peak_hour_consumption),
                off_hour_consumption: Some(off_hour_consumption),
                peak_hour_injection: Some(peak_hour_injection),
                off_hour_injection: Some(kwh),
            }),
            _ => Ok(partial),
        },
        _ => Ok(partial),
    }
}

fn parse_lines<T>(lines: T) -> Result<Option<CompleteP1Measurement>, Box<dyn Error>>
where
    T: IntoIterator,
    T::Item: Borrow<str>,
{
    let mut partial = PartialP1Measurement {
        timestamp: None,
        peak_hour_consumption: None,
        off_hour_consumption: None,
        peak_hour_injection: None,
        off_hour_injection: None,
    };
    for line in lines.into_iter() {
        match complete_p1_measurement(step_partial_p1_measurement(partial, line.borrow())?) {
            Ok(complete) => return Ok(Some(complete)),
            Err(new_partial) => partial = new_partial,
        }
    }
    return Ok(None);
}

fn call_sqlite3 (input: &str) -> String {
    let mut cmd = Command::new("sqlite3");
    let process = match cmd
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn sqlite3: {}", why),
        Ok(process) => process,
    };

    // stdin has type Option<ChildStdin>, but since we know this instance
    // must have one, we can directly unwrap it.
    match process.stdin.unwrap().write_all(input.as_bytes()) {
        Err(why) => panic!("couldn't write to sqlite3 stdin: {}", why),
        Ok(_) => {}
    }

    // Because stdin does not live after the above calls, it is drop-ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise sqlite3 wouldn't start processing the
    // input we just sent.

    // The stdout field also has type Option<ChildStdout> so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read sqlite3 stdout: {}", why),
        Ok(_) => {},
    }
    return s;
}

fn main() {
    println!("Hello, world!");
    println!(
        "sqlite3: {}",
        call_sqlite3(".mode json\nCREATE TABLE t (a INTEGER, b STRING);INSERT INTO t VALUES (123, 'a string|gnirts a');SELECT * FROM t;"));
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
    let mut idx = 0;
    for line in "\n0-0:1.0.0(241025191816S)\n\n1-0:1.8.1(002654.919*kWh)\n\n1-0:1.8.2(002420.293*kWh)\n\n1-0:2.8.1(006254.732*kWh)\n\n1-0:2.8.2(002457.202*kWh)".lines() {
        idx = idx + 1;
        println!("{}: {}", idx, line);
    }
}
