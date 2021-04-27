package main

import (
	"fmt"
	"testing"
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

func TestGetManhattan(t *testing.T) {
	s := "R75,D30,R83,U83,L12,D49,R71,U7,L72\n"
	s += "U62,R66,U55,R34,D71,R55,D58,R83";

	paths1, paths2 := process_data(s)
	assertEq(t, 159, get_manhattan(paths1, paths2), "t1")

	s = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n"
	s += "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

	paths1, paths2 = process_data(s)
	assertEq(t, 135, get_manhattan(paths1, paths2), "t1")
}

func TestLatency(t *testing.T) {
	s := "R75,D30,R83,U83,L12,D49,R71,U7,L72\n"
	s += "U62,R66,U55,R34,D71,R55,D58,R83";

	paths1, _ := process_data(s)
	wps, _ := make_wirepath_set(paths1, no_op, nil)

	assertEq(t, 75, wps[0].get_latency(), "t1")
	assertEq(t, 482, wps[8].get_latency(), "t2")
}

func TestGetLatency(t *testing.T) {
	s := "R8,U5,L5,D3\n"
	s += "U7,R6,D4,L4"

	p1, p2 := process_data(s)
	assertEq(t, 30, get_latency(p1,p2), "t1")

	s = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n"
	s += "U62,R66,U55,R34,D71,R55,D58,R83"

	p1, p2 = process_data(s)
	assertEq(t, 610, get_latency(p1, p2), "t2")

	s = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n"
	s += "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"

	p1, p2 = process_data(s)
	assertEq(t, 410, get_latency(p1, p2), "t3")
}

var result int

func BenchmarkDay3(b *testing.B) {
 	var r int

	for i := 0; i < b.N; i++ {
		r, _ = Day3("../../input")
	}

	result = r
}

func BenchmarkGetManhattan(b *testing.B) {
	var r int

	f_data := process_file("../../input")
	p1, p2 := process_data(f_data)

	for i := 0; i < b.N; i++ {
		r = get_manhattan(p1, p2)
	}

	result = r
}

func BenchmarkGetLatency(b *testing.B) {
	var r int

	f_data := process_file("../../input")
	p1, p2 := process_data(f_data)

	for i := 0; i < b.N; i++ {
		r = get_latency(p1, p2)
	}

	result = r
}

func BenchmarkProcessData(b *testing.B) {
	var p1 []string

	f_data := process_file("../../input")
	
	for i := 0; i < b.N; i++ {
		p1, _ = process_data(f_data)

	}

	result = int(p1[0][0])

}
