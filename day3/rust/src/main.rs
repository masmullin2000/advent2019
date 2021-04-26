use anyhow::Result;

extern crate day3_lib;
use day3_lib::*;

fn main() -> Result<()> {
    match day3("../input") {
        Ok(v) => println!("distance is {}", v),
        Err(_) => {}
    }
    Ok(())
}
