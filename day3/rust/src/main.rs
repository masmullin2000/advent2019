use anyhow::Result;

extern crate day3_lib;
use day3_lib::*;

fn main() -> Result<()> {
    match day3("../input") {
        Ok((man, lat)) => {
            println!("manhattan is {}", man);
            println!("latency is {}", lat);
        }
        Err(_) => {}
    }
    Ok(())
}
