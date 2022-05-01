use anyhow::Result;
use clap::Parser;
use scripts::{config, mail};

#[derive(Parser)]
#[clap()]
struct Args {}

fn main() -> Result<()> {
    let config = config::read();
    let transport = mail::new_transport(config.smtp)?;
    Ok(())
}
