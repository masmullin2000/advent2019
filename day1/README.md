# NOTES #

## RUST ##

### Basic Code (10/10) ###
Code is simple and straight forward

- Easy to read from a file into a string
- easy to parse the string into lines
- easy to change the lines into u64

### Test (10/10) ###
Testing Is simple

Don't have to use #[cfg(test)] and then have tests as a module
but it's standard operating procedure in rust
need to use super::\*; which is a bit weird, but OK

[dev-dependencies] needed for benchmarks pulls in criterion for tests as well
annoying because it compiles and downloads 92 other crates
((marks off benchmarks -0.5))

### Benchmarks (6.5/10) ###
Easy once docs are read
Criterion can't benchmark inside of binaries (-1)
Had to split the day1 into a lib and a binary, then benchmark the library
annoying for benching a small set of functionality because we need to split the
result into something more suitable for a large binary (already calced as no bench binary)

Cannot benchmark non-public functions (serious problem -2)
Note: Can get around using config flags, but still big problem

Cool graphics and info on previous runs via gnuplot (bonus +1)

## Overall: 27.5/30 ##

## GO ##

### BASIC CODE (8.25/10)###

Pretty easy, but requires more libraries than Rust -0.5
Project layout is sort of annoying,
- doesn't need to be done the go way, but "should be" - 0.25
- needed to do a Makefile
Unused variables treated as errors rather than warnings -1

Super fast compile: +1

### TESTING (9.5/10)###
Testing is very simple, but cannot be done in-file -0.5
Must write bespoke asserts -0.25

### BENCHMARKING (10/10) ###
Super easy

### Overall: 27.5/30 ###

## PERFORMANCE ##
Go = 16311 ns for Day1 (16.311 us)
Rust = 5.148 us

GO/Rust = 3.16 = +1.6 bonus to Rust

Extra Bonus to go for standalone binaries +1

Final Results:
Rust: 29.1
Go 28.5

