//! `cargo run --features gates --example gates -- log.dlz 127.0.0.1:0`
use deleuze::{
    Completion, Log, Options,
    gates::{Gate, GateOptions},
};
use std::io::Write;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let path = args.next().ok_or("supply a log file path")?;
    let address = args
        .next()
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .into_string()
        .map_err(|_| "address must be UTF-8")?;
    let mode = args
        .next()
        .unwrap_or_else(|| "written".into())
        .into_string()
        .map_err(|_| "completion must be UTF-8")?;
    let completion = match mode.as_str() {
        "written" => Completion::Written,
        "synced" => Completion::Synced,
        _ => return Err("completion must be written or synced".into()),
    };
    if args.next().is_some() {
        return Err("unexpected extra arguments".into());
    }
    let log = Arc::new(Log::open(
        path,
        Options {
            completion,
            ..Options::default()
        },
    )?);
    let gate = Gate::new(log, GateOptions::default())?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("{}", listener.local_addr()?);
    std::io::stdout().flush()?;
    gate.serve(listener, async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await?;
    Ok(())
}
