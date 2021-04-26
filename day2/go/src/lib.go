package main

import (
	"fmt"
	"strings"
	"strconv"
	"io/ioutil"
)

func process_file(file string, delim string) []int {
	var rc []int
	f_data, err := ioutil.ReadFile(file)
	if err != nil {
		panic("didn't read file")
	}

	lines_str := strings.Trim(string(f_data),"\n")
	lines := strings.Split(lines_str,delim)

	for _,l := range lines {
		if l != "" {
			ret, err := strconv.Atoi(l)
			if err != nil {
				fmt.Println("error on ", l, err)
				panic("parse error")
			}
			rc = append(rc, ret)
		}
	}

	return rc
}

func get_val(codes []int, pos int) int {
	return codes[codes[pos]]
}

func perform_op(codes []int, idx int) bool {
	ip := idx * 4
	if codes[ip] == 99 {
		return false
	} else if codes[ip] != 1 && codes[ip] != 2 {
		panic("Unknown Op")
	}

	val1 := get_val(codes, ip+1)
	val2 := get_val(codes, ip+2)
	store := codes[ip+3]

	if codes[ip] == 1 {
		codes[store] = val1 + val2
	} else {
		codes[store] = val1 * val2
	}

	return true
}

func _process_codes(codes []int, noun int, verb int, set_nv bool) int {
	i := 0

	if set_nv == true {
		codes[1] = noun
		codes[2] = verb
	}

	for {
		if true != perform_op(codes, i) {
			break
		}
		i++
	}

	return codes[0]
}

func process_codes(codes []int, noun int, verb int, set_nv bool) int {
	ccodes := make([]int, len(codes))
	copy(ccodes, codes)
	return _process_codes(ccodes, noun, verb, set_nv)
}

func find_noun_verb(out int, codes []int, noun int, verb int, set_nv bool) bool {
	rc := process_codes(codes, noun, verb, true)
	if rc == out {
		return true
	}

	return false
}

func Day2(file string) (int, int, int) {
	var rc int
	var n int
	var v int

	codes := process_file(file,",")
	
	rc = process_codes(codes, 12, 2, true)

	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			found := find_noun_verb(19690720, codes, noun, verb, true)
			if found == true {
				n = noun
				v = verb
				return rc,n,v;
			}
		}
	}

	return rc,-1,-1
}