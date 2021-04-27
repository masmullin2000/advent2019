package main

import (
	"testing"
	"fmt"
)

func assertEqArrays(t *testing.T, a []int, b []int, err_message string) {
	if len(a) != len(b) {
		fmt.Println(len(a), len(b))
		t.Fatal()
	}

	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			fmt.Println(a[i], b[i])
			t.Fatal(err_message)
		}
	}
}

func assertEq(t *testing.T, a interface{}, b interface{}, err_message string) {
	if a != b {
		fmt.Println(a, b)
		t.Fatal(err_message)
	}
}

// I had the proper testing, but deleted them by accident
// not going to rewrite

func TestCountValidExtra(t *testing.T) {
	fmt.Println("ValidExtra")
	assertEq(t, 0, count_valid(111111,111111, validate_extra), "t1")
	assertEq(t, 0, count_valid(111111,111112, validate_extra), "t2")
	assertEq(t, 0, count_valid(1,99999,validate_extra), "t3")
}

var result int

func BenchmarkDay4_1(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 1)
	}

	result = r
}

func BenchmarkDay4_6(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 6)
	}

	result = r
}


func BenchmarkDay4_12(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 12)
	}

	result = r
}

func BenchmarkDay4_18(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 18)
	}

	result = r
}

func BenchmarkDay4_24(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 24)
	}

	result = r
}

func BenchmarkDay4_48(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 48)
	}

	result = r
}

func BenchmarkDay4_96(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 96)
	}

	result = r
}

func BenchmarkDay4_10000(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 10000)
	}

	result = r
}

func BenchmarkDay4_50000(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 50000)
	}

	result = r
}

func BenchmarkDay4_100000(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 100000)
	}

	result = r
}

func BenchmarkDay4_1000000(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_ = Day4("256310-732736", 1000000)
	}

	result = r
}