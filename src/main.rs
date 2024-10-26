use std::num::ParseFloatError;
use std::str::FromStr;
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
        assert_eq!(parse_kwh("prefix(12.34*kWh)", "mismtach"), Ok(None))
    }

    #[test]
    fn parse_kwh_bad_float_format_expect_err() {
        assert!(parse_kwh("prefix(bad-float*kWh)", "prefix(").is_err())
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
}
