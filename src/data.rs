use std::fmt::Display;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

/*
CREATE TABLE data_202208 (
    timestamp INTEGER PRIMARY KEY ASC,
    pv2012_kWh FLOAT,
    pv2022_kWh FLOAT,
    peak_conso_kWh FLOAT,
    off_conso_kWh FLOAT,
    gas_m3 FLOAT,
    water_m3 FLOAT
  );
CREATE TABLE data_202303 (
    timestamp INTEGER PRIMARY KEY ASC,
    pv2012_kWh FLOAT,
    pv2022_kWh FLOAT,
    peak_conso_kWh FLOAT,
    off_conso_kWh FLOAT,
    peak_inj_kWh FLOAT,
    off_inj_kWh FLOAT,
    gas_m3 FLOAT,
    water_m3 FLOAT
  );
 */

#[derive(Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct Data202208 {
    timestamp: i64,
    pv2012_kWh: Option<f64>,
    pv2022_kWh: Option<f64>,
    peak_conso_kWh: Option<f64>,
    off_conso_kWh: Option<f64>,
    gas_m3: Option<f64>,
    water_m3: Option<f64>,
}

#[derive(Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct Data202303 {
    timestamp: i64,
    pv2012_kWh: Option<f64>,
    pv2022_kWh: Option<f64>,
    peak_conso_kWh: Option<f64>,
    off_conso_kWh: Option<f64>,
    peak_inj_kWh: Option<f64>,
    off_inj_kWh: Option<f64>,
    gas_m3: Option<f64>,
    water_m3: Option<f64>,
}

fn some_str_to_result<B, C, F>(a: Option<&str>, f: F) -> Result<Option<B>, String>
where
    F: FnOnce(&str) -> Result<B, C>,
    C: Display,
{
    match a {
        None => Ok(None),
        Some(s) => {
            if s.trim().len() == 0 {
                Ok(None)
            } else {
                f(s).map(Some).map_err(|e| format!("{}", e))
            }
        }
    }
}

pub fn select_data_202208(cmd: &str) -> Result<Vec<Data202208>, String> {
    let sql_output = call_sqlite3(
        cmd,
        ".mode list\nselect count(*) from data_202208;\nselect timestamp, pv2012_kWh, pv2022_kWh, peak_conso_kWh, off_conso_kWh, gas_m3, water_m3 from data_202208;",
    );
    let mut info = sql_output.lines();
    let count = match info.next().map(usize::from_str) {
        Some(Ok(count)) => count,
        None => {
            return Err("No row count for data_202208".to_string());
        }
        Some(Err(_)) => {
            return Err("Malformed row count dor data_202208".to_string());
        }
    };
    let mut result = Vec::<Data202208>::with_capacity(count);
    for line in info {
        let mut cols = line.split("|");
        let timestamp = match cols.next().map(i64::from_str) {
            Some(Ok(ts)) => ts,
            None => {
                return Err("No timestamp".to_string());
            }
            Some(Err(_)) => return Err("Unable to parse timestamp".to_string()),
        };
        result.push(Data202208 {
            timestamp: timestamp,
            pv2012_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            pv2022_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            peak_conso_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            off_conso_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            gas_m3: some_str_to_result(cols.next(), f64::from_str)?,
            water_m3: some_str_to_result(cols.next(), f64::from_str)?,
        })
    }
    return Ok(result);
}

pub fn select_data_202303(cmd: &str) -> Result<Vec<Data202303>, String> {
    let sql_output = call_sqlite3(
        cmd,
        ".mode list\nselect count(*) from data_202303;\nselect timestamp, pv2012_kWh, pv2022_kWh, peak_conso_kWh, peak_inj_kWh, off_conso_kWh, off_inj_kWh, gas_m3, water_m3 from data_202303;",
    );
    let mut info = sql_output.lines();
    let count = match info.next().map(usize::from_str) {
        Some(Ok(count)) => count,
        None => {
            return Err("No row count for data_202208".to_string());
        }
        Some(Err(_)) => {
            return Err("Malformed row count dor data_202208".to_string());
        }
    };
    let mut result = Vec::<Data202303>::with_capacity(count);
    for line in info {
        let mut cols = line.split("|");
        let timestamp = match cols.next().map(i64::from_str) {
            Some(Ok(ts)) => ts,
            None => {
                return Err("No timestamp".to_string());
            }
            Some(Err(_)) => return Err("Unable to parse timestamp".to_string()),
        };
        result.push(Data202303 {
            timestamp: timestamp,
            pv2012_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            pv2022_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            peak_conso_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            peak_inj_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            off_conso_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            off_inj_kWh: some_str_to_result(cols.next(), f64::from_str)?,
            gas_m3: some_str_to_result(cols.next(), f64::from_str)?,
            water_m3: some_str_to_result(cols.next(), f64::from_str)?,
        })
    }
    return Ok(result);
}

pub fn call_sqlite3(cmd: &str, input: &str) -> String {
    let process = match Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
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
        Ok(_) => {}
    }
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = call_sqlite3("cat", "hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn count_and_select_data_202208() {
        let result = select_data_202208(
            "echo '2\n1356994800|487.0|0.0|82313.0|35983.0|9203.0|-393.0\n1359673200|553.0||82564.0|36184.0|9685.0|-385.0'"
        ).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Data202208 {
                timestamp: 1356994800,
                pv2012_kWh: Some(487.0),
                pv2022_kWh: Some(0.0),
                peak_conso_kWh: Some(82313.0),
                off_conso_kWh: Some(35983.0),
                gas_m3: Some(9203.0),
                water_m3: Some(-393.0)
            }
        );
        assert_eq!(
            result[1],
            Data202208 {
                timestamp: 1359673200,
                pv2012_kWh: Some(553.0),
                pv2022_kWh: None,
                peak_conso_kWh: Some(82564.0),
                off_conso_kWh: Some(36184.0),
                gas_m3: Some(9685.0),
                water_m3: Some(-385.0)
            }
        );
    }

    #[test]
    fn count_and_select_data_202303() {
        let result = select_data_202303(
            "echo '2\n1695485100|50621.3|3579.4|||630.0|1189.4|28973.5|867.5\n1695537420||3579.9||||||'"
        ).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Data202303 {
                timestamp: 1695485100,
                pv2012_kWh: Some(50621.3),
                pv2022_kWh: Some(3579.4),
                peak_conso_kWh: None,
                off_conso_kWh: Some(630.0),
                peak_inj_kWh: None,
                off_inj_kWh: Some(1189.4),
                gas_m3: Some(28973.5),
                water_m3: Some(867.5)
            }
        );
        assert_eq!(
            result[1],
            Data202303 {
                timestamp: 1695537420,
                pv2012_kWh: None,
                pv2022_kWh: Some(3579.9),
                peak_conso_kWh: None,
                peak_inj_kWh: None,
                off_conso_kWh: None,
                off_inj_kWh: None,
                gas_m3: None,
                water_m3: None
            }
        );
    }
}
