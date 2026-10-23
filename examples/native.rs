//! Run with `cargo run --example native -- /path/to/new-or-clean-log.dlz`.
use deleuze::{Log, Options, Query};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os().nth(1).ok_or("supply a log file path")?;
    let log = Log::open(path, Options::default())?;
    let record = log.append(String::from("ordinary producer output"))?;
    println!("write sequence: {}", record.seq());
    for entry in log.query(Query::default())?.entries {
        println!("{} {:?}", entry.record.seq(), entry.properties);
    }
    Ok(())
}
