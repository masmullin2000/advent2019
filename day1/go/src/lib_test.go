package main

import (
	"testing"
	//"fmt"
)

var result int

func assertEq(t *testing.T, a interface{}, b interface{}, err_message string) {
	if a != b {
		t.Fatal(err_message)
	}
}

func TestFuelCalc(t *testing.T) {
	assertEq(t, 2, fuel_calculation(12), "t1")
	assertEq(t, 2, fuel_calculation(14), "t2")
	assertEq(t, 966, fuel_calculation(1969), "t3")
	assertEq(t, 50346, fuel_calculation(100756), "t4")
}

func BenchmarkFuelCalc(b *testing.B) {
	var r int
	for i := 0; i < b.N; i++ {
		r = fuel_calculation(69)
	}
	result = r
}

func BenchmarkDay1(b *testing.B) {
	var r int
	for i := 0; i < b.N; i++ {
		r = Day1("../input")
	}

	result = r
}