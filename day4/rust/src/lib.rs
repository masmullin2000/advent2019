use anyhow::Result;
use std::thread;

type FuncVal = fn(&str) -> bool;

fn validate(password: &str) -> bool {
    let mut rc = false;
    if password.len() < 6 {
    	return false;
    }

    let mut prev = password.chars().nth(0).unwrap();
    for i in 1..6 {
    	let test = password.chars().nth(i).unwrap();
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

fn validate_extra(password: &str) -> bool {
	let mut rc = false;
	if password.len() < 6 {
		return false;
	}

	let mut m = '0';
	let mut prev = password.chars().nth(0).unwrap();
	for i in 1..6 {
    	let test = password.chars().nth(i).unwrap();
    	if test < prev {
    		return false;
    	}

    	if test == prev {
    		if test == m {
    			rc = false;
    		} else if m == '0' || rc == false {
    			rc = true;
    			m = test;
    		}
    	}
    	prev = test;
    }

    rc
}

fn count_valid(start: i64, end: i64, val_func: FuncVal) -> i64 {

	println!("from {} to {}", start, end);
    let mut count = 0;

    for i in start..=end {
    	let password = i.to_string();
    	if val_func(&password) {
    		count += 1;
    	}
    }

    count
}

fn dowork(mut min: i64, max: i64, amt: i64, parallel: i64, val_func: FuncVal) -> i64 {
    let mut handles: Vec<thread::JoinHandle<i64>> = Vec::new();

    for _ in 0..parallel {
        let handle = thread::spawn(move || {
            let end = if max - (min + amt) < parallel {
                max
            } else {
                min + amt
            };

            count_valid(min, end, val_func)
        });

        min += amt;
        handles.push(handle);
    }

    let mut result = 0;
    for _ in 0..parallel {
        result += handles.pop().unwrap().join().unwrap_or(0);
    }

    result
}

pub fn day4(data: &str, mut parallel: i64) -> Result<(i64, i64)> {
    let d: Vec<&str> = data.split('-').collect();

    let min: i64 = d[0].parse()?;
    let max: i64 = d[1].parse()?;

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
		assert_eq!(true, validate("111111"));
		assert_eq!(false, validate("223450"));
		assert_eq!(false, validate("123789"));
	}

	#[test]
	fn test_count_valid() {
		assert_eq!(1, count_valid(111111,111111, validate));
		assert_eq!(2, count_valid(111111,111112, validate));
		assert_eq!(0, count_valid(1,99999, validate));
		assert_eq!(979, count_valid(256310, 732736, validate));
	}

	#[test]
	fn test_valid_extra() {
		assert_eq!(false, validate_extra("111111"));
		assert_eq!(false, validate_extra("223450"));
		assert_eq!(false, validate_extra("123789"));

		assert_eq!(true, validate_extra("112233"));
		assert_eq!(false, validate_extra("123444"));
		assert_eq!(true, validate_extra("111122"));
		assert_eq!(true, validate_extra("112222"));
		assert_eq!(false, validate_extra("111222"));
	}

	#[test]
	fn test_count_valid_extra() {
		assert_eq!(0, count_valid(111111,111111, validate_extra));
		assert_eq!(0, count_valid(111111,111112, validate_extra));
		assert_eq!(0, count_valid(1,99999, validate_extra));
		assert_eq!(635, count_valid(256310, 732736, validate_extra));
	}
}