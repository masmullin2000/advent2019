/// Performance modifications using normal Rust Strings instead of needing to resort to arrays of u8
///
/// Notes
/// - still uses `itoa::fmt` to format a string to an existing String buffer quickly -- it's faster
//    than calling `write!(...)`
/// - reuses a `String`, clearing it rather than re-allocating
/// - uses `chars()` iterator to do a single pass along the string
/// - using generic `<F>` parameter rather than function pointers, to permit monomorphization
///
/// On my machine, it's slightly faster than the [u8] algorithm too. Might be bounds checking.
///
/// Original: https://github.com/masmullin2000/advent2019/blob/main/day4/rust_async/src/lib.rs
///
/// And credits to commenters Dylan Foxx et al on the video https://www.youtube.com/watch?v=E-47VLwMY_U for comments 
/// originally pointing out how to use the chars() iterator.


use anyhow::Result;
use tokio::spawn;

fn validate(password: &str) -> bool {
    // we want exactly 6 bytes
    // each char should be only one byte
    if password.len() != 6 {
        return false;
    }

    let mut rc = false;
    
    // using iterator, take first character, then compare remaining ones
    let mut chars_iter = password.chars();
    let mut prev = chars_iter.next().unwrap();

    for test in chars_iter.take(5) {
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
    // we want exactly 6 bytes
    // each char should be only one byte
    if password.len() != 6 {
        return false;
    }
    let mut rc = false;

    // using iterator, take first character, then compare remaining ones
    let mut chars_iter = password.chars();
    let mut prev = chars_iter.next().unwrap();

    let mut m = '0';
    for test in chars_iter.take(5) {
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

fn count_valid<F: Fn(&str) -> bool>(start: i64, end: i64, val_func: F) -> i64 {
    let mut count = 0;

    // re-usable string, to avoid allocating repeatedly
    let mut password = String::new();
    for i in start..=end {
        // clear and append the password
        password.clear();
        itoa::fmt(&mut password, i).unwrap();

        // slower way, via the normal formatter
        //write!(password, "{}", i).unwrap();

        if val_func(&password) {
            count += 1;
        }
    }

    count
}

#[tokio::main]
async fn dowork<F>(mut min: i64, max: i64, amt: i64, parallel: i64, val_func: F) -> i64
where
    F: 'static + Fn(&str) -> bool + Send + Copy
{
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
        assert_eq!(true, validate("111111"));
        assert_eq!(false, validate("223450"));
        assert_eq!(false, validate("123789"));
    }

    // #[test]
    // fn test_count_valid() {
    //  assert_eq!(1, count_valid(111111,111111, validate));
    //  assert_eq!(2, count_valid(111111,111112, validate));
    //  assert_eq!(0, count_valid(1,99999, validate));
    //  assert_eq!(979, count_valid(256310, 732736, validate));
    // }

    // #[test]
    // fn test_valid_extra() {
    //  assert_eq!(false, validate_extra("111111"));
    //  assert_eq!(false, validate_extra("223450"));
    //  assert_eq!(false, validate_extra("123789"));

    //  assert_eq!(true, validate_extra("112233"));
    //  assert_eq!(false, validate_extra("123444"));
    //  assert_eq!(true, validate_extra("111122"));
    //  assert_eq!(true, validate_extra("112222"));
    //  assert_eq!(false, validate_extra("111222"));
    // }

    // #[test]
    // fn test_count_valid_extra() {
    //  assert_eq!(0, count_valid(111111,111111, validate_extra));
    //  assert_eq!(0, count_valid(111111,111112, validate_extra));
    //  assert_eq!(0, count_valid(1,99999, validate_extra));
    //  assert_eq!(635, count_valid(256310, 732736, validate_extra));
    // }
}
