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
		t.Fatal(err_message)
	}
}

func TestProcessCodes(t *testing.T) {
	codes := []int{1,0,0,0,99}
	rc := _process_codes(codes,0,0,false)
 	assertEqArrays(t, []int{2,0,0,0,99}, codes, "t1")
	assertEq(t, rc, 2, "t2")

	codes = []int{2,3,0,3,99}
	rc = _process_codes(codes,0,0,false)
	assertEqArrays(t, []int{2,3,0,6,99}, codes, "t3")
	
	codes = []int{2,4,4,5,99,0}
	rc = _process_codes(codes,0,0,false)
	assertEqArrays(t, []int{2,4,4,5,99,9801}, codes, "t4")

	codes = []int{1,1,1,4,99,5,6,0,99}
	rc = _process_codes(codes,0,0,false)
	assertEqArrays(t, []int{30,1,1,4,2,5,6,0,99}, codes, "t5")
}

var result int

func BenchmarkDay2(b *testing.B) {
	var r int

	for i := 0; i < b.N; i++ {
		r,_,_ = Day2("../../input")
	}

	result = r
}