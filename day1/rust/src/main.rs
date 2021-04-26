use anyhow::Result;

extern crate day1_lib;
use day1_lib::*;

fn main() -> Result<()> {
    let fuel = day1("input")?;
    println!("Fuel Required: {}", fuel);

    Ok(())
}
