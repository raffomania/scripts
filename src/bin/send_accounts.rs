use clap::Parser;

#[derive(Parser)]
#[clap()]
struct Args {}

fn main() -> anyhow::Result<()> {
    println!("hi");
    Ok(())
}
