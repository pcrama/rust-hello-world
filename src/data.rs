use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn call_sqlite3(cmd: &str, input: &str) -> String {
    let mut cmd = Command::new(cmd);
    let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
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
}
