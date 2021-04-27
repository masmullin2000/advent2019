use anyhow::Result;

extern crate day4_lib;
use day4_lib::*;

fn main() -> Result<()> {
    let (r1,r2) = day4("256310-732736", 12)?;

    println!("{} {}", r1, r2);

    Ok(())
}
