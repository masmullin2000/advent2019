use std::fs;
use anyhow::{anyhow, Result};
//use std::str::FromStr;

fn process_file(file: &str) -> Result<Vec<u64>> {
	let f_data = fs::read_to_string(file).expect("didn't find the file");

	let ops: Vec<u64> = f_data
		.split(',').map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	Ok(ops)
}

fn get_val(codes: &Vec<u64>, loc: usize) -> u64 {
	let idx = codes[loc] as usize;
	if codes.len() <= idx {
		panic!("operation index off the stack {}::{}", codes.len(), idx);
	}
	codes[idx]
}

fn perform_op(codes: &mut Vec<u64>, ip: usize) -> Option<usize> {
	if codes.len() <= ip {
		panic!("instruction pointer off stack {}::{}", codes.len(), ip);
	} else if codes[ip] == 99 {
		return None;
	} else if codes[ip] != 1 && codes[ip] != 2 {
		panic!("Illegal op {}", codes[ip])
	}

	let val1 = get_val(codes, ip+1);
	let val2 = get_val(codes, ip+2);
	let sto = codes[ip+3] as usize;

	if codes.len() <= sto {
		panic!("Storage location off stack {}::{}", codes.len(), sto);
	}

	if codes[ip] == 1 {
		codes[sto] = val1 + val2;
	} else if codes[ip] == 2 {
		codes[sto] = val1 * val2;
	}

	Some(ip + 4)
}

fn _process_codes(codes: &mut Vec<u64>, noun: u64, verb: u64, set_nv: bool) -> Result<u64> {
	if set_nv == true {
		codes[1] = noun;
		codes[2] = verb;
	}

	let mut ip = 0usize;

	loop{
		match perform_op(codes, ip) {
			Some(i) => ip = i,
			None => break,
		}
	}

	Ok(codes[0])
}

fn process_codes(codes: &Vec<u64>, noun: u64, verb: u64, set_nv: bool) -> Result<u64> {
	let mut ccodes = codes.clone();
	_process_codes(&mut ccodes, noun, verb, set_nv)
}

fn find_noun_verb(out: u64, codes: &Vec<u64>, noun: u64, verb: u64, set_nv: bool) -> bool {
	let rc = process_codes(codes, noun, verb, set_nv).unwrap_or(out-1);
	if rc == out {
		return true;
	}

	false
} 

pub fn day2(file: &str) -> Result<(u64,u64,u64)> {
	let codes = process_file(file)?;

	let val = process_codes(&codes,12,2,true)?;

	for n in 0..=99 {
		for v in 0..=99 {
			if find_noun_verb(19690720, &codes, n, v, true) {
				return Ok((val, n, v));
			}
		}
	}

	Err(anyhow!("Didn't find values"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process_codes() {
		let mut codes = vec![1,0,0,0,99];
		_process_codes(&mut codes,0,0,false).unwrap();
		assert_eq!(codes, vec![2,0,0,0,99]);

		codes = vec![2,3,0,3,99];
		_process_codes(&mut codes,0,0,false).unwrap();
		assert_eq!(codes, vec![2,3,0,6,99]);

		codes = vec![2,4,4,5,99,0];
		_process_codes(&mut codes,0,0,false).unwrap();
		assert_eq!(codes, vec![2,4,4,5,99,9801]);
		
		codes = vec![1,1,1,4,99,5,6,0,99];
		_process_codes(&mut codes,0,0,false).unwrap();
		assert_eq!(codes, vec![30,1,1,4,2,5,6,0,99]);
	}
}