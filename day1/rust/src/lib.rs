use std::fs;
use anyhow::Result;
use std::str::FromStr;

fn fuel_calculation(mass: i64) -> i64 {
    let mut fuel = (mass / 3) - 2;
    if fuel < 0 {
        fuel = 0;
    } else {
        fuel += fuel_calculation(fuel);
    }

    fuel
}

pub fn day1(file: &str) -> Result<i64>  {
    let f_data = fs::read_to_string(file)?;
    let lines = f_data.lines();
    let mut fuel: i64 = 0;

    for l in lines {
        let mass = i64::from_str(l)?;
        fuel += fuel_calculation(mass);
    }

    Ok(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_calculation() {
        assert_eq!(2, fuel_calculation(12));
        assert_eq!(2, fuel_calculation(14));
        assert_eq!(966, fuel_calculation(1969));
        assert_eq!(50346, fuel_calculation(100756));
    }
}

#[cfg(feature = "bench")]
pub fn fc(mass: i64) -> i64 {
    fuel_calculation(mass)
}