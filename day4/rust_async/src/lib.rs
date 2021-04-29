use anyhow::Result;
use std::thread;

extern crate itoa;

use itoa::*;

use tokio::spawn;

type FuncVal = fn(&[u8]) -> bool;

fn validate(password: &[u8]) -> bool {
    let mut rc = false;
    if password.len() < 6 {
        return false;
    }

    // when using string
    // let mut prev = password.chars().nth(0).unwrap();
    // for i in 1..6 {
    // 	let test = password.chars().nth(i).unwrap();

    let mut prev = password[0];
    for i in 1..6 {
        let test = password[i];

        if test < prev {
            return false;
        }

        if test == prev {
            rc = true;
        }

        prev = test;
    }

    rc
}

fn validate_extra(password: &[u8]) -> bool {
    let mut rc = false;
    if password.len() < 6 {
        return false;
    }

    // let mut m = '0';
    // let mut prev = password.chars().nth(0).unwrap();
    // for i in 1..6 {
    //    	let test = password.chars().nth(i).unwrap();
    let mut m = '0' as u8;
    let mut prev = password[0];
    for i in 1..6 {
        let test = password[i];
        if test < prev {
            return false;
        }

        if test == prev {
            if test == m {
                rc = false;
            } else if m == '0' as u8 || rc == false {
                rc = true;
                m = test;
            }
        }
        prev = test;
    }

    rc
}

fn count_valid(start: i64, end: i64, val_func: FuncVal) -> i64 {
    let mut count = 0;

    for i in start..=end {
        //let password = i.to_string();

        let mut bytes = [0u8; 6];
        itoa::write(&mut bytes[..], i);
        if val_func(&bytes) {
            count += 1;
        }
    }

    count
}

#[tokio::main]
async fn dowork(mut min: i64, max: i64, amt: i64, parallel: i64, val_func: FuncVal) -> i64 {
    let mut tasks = vec![];

    for _ in 0..parallel {
        let t = spawn(async move {
            let end = if max - (min + amt) < parallel {
                max
            } else {
                min + amt
            };

            count_valid(min, end, val_func)
        });

        min += amt;
        tasks.push(t);
    }

    let mut result = 0;
    for task in tasks {
        result += task.await.unwrap();
    }

    result
}

pub fn day4(data: &str, mut parallel: i64) -> Result<(i64, i64)> {
    let d: Vec<&str> = data.split('-').collect();

    let min: i64 = d[0].parse()?;
    let max: i64 = d[1].parse()?;

    if parallel > 65535 {
        parallel = 65535;
    }

    let mut amt = (max - min) / parallel;

    if amt <= 0 {
        amt = 1;
        parallel = max - min;
    }

    let r1 = dowork(min, max, amt, parallel, validate);
    let r2 = dowork(min, max, amt, parallel, validate_extra);

    Ok((r1, r2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(true, validate("111111".as_bytes()));
        assert_eq!(false, validate("223450".as_bytes()));
        assert_eq!(false, validate("123789".as_bytes()));
    }

    // #[test]
    // fn test_count_valid() {
    // 	assert_eq!(1, count_valid(111111,111111, validate));
    // 	assert_eq!(2, count_valid(111111,111112, validate));
    // 	assert_eq!(0, count_valid(1,99999, validate));
    // 	assert_eq!(979, count_valid(256310, 732736, validate));
    // }

    // #[test]
    // fn test_valid_extra() {
    // 	assert_eq!(false, validate_extra("111111"));
    // 	assert_eq!(false, validate_extra("223450"));
    // 	assert_eq!(false, validate_extra("123789"));

    // 	assert_eq!(true, validate_extra("112233"));
    // 	assert_eq!(false, validate_extra("123444"));
    // 	assert_eq!(true, validate_extra("111122"));
    // 	assert_eq!(true, validate_extra("112222"));
    // 	assert_eq!(false, validate_extra("111222"));
    // }

    // #[test]
    // fn test_count_valid_extra() {
    // 	assert_eq!(0, count_valid(111111,111111, validate_extra));
    // 	assert_eq!(0, count_valid(111111,111112, validate_extra));
    // 	assert_eq!(0, count_valid(1,99999, validate_extra));
    // 	assert_eq!(635, count_valid(256310, 732736, validate_extra));
    // }
}
