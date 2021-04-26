use anyhow::Result;

extern crate day2_lib;
use day2_lib::*;

fn main() -> Result<()> {
    let (rc,noun,verb) = day2("../input")?;
    
    println!("result {}", rc);
    println!("noun verb is {:02}{:02}", noun, verb);

    Ok(())
}