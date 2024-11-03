pub mod data;
pub mod p1_meter;

fn main() {
    println!("Hello, world!");
    println!(
        "sqlite3: {}",
        data::call_sqlite3("sqlite3", ".mode json\nCREATE TABLE t (a INTEGER, b STRING);INSERT INTO t VALUES (123, 'a string|gnirts a');SELECT * FROM t;"));
    let mut idx = 0;
    for line in "\n0-0:1.0.0(241025191816S)\n\n1-0:1.8.1(002654.919*kWh)\n\n1-0:1.8.2(002420.293*kWh)\n\n1-0:2.8.1(006254.732*kWh)\n\n1-0:2.8.2(002457.202*kWh)".lines() {
        idx = idx + 1;
        println!("{}: {}", idx, line);
    }
}
