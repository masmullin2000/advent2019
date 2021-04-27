package main

import (
	"strings"
	"strconv"
	"sync"
)

func validate(password string) bool {
	rc := false
	if len(password) < 6 {
		return false
	}

	prev := rune(password[0])
	for i := 1; i < 6; i++ {
		if rune(password[i]) < prev {
			return false
		}
		if rune(password[i]) == prev {
			rc = true
		}
		prev = rune(password[i])
	}

	return rc
}

func validate_extra(password string) bool {
	rc := false
	if len(password) < 6 {
		return false
	}

	match := '0'
	prev := rune(password[0])
	for i := 1; i < 6; i++ {
		test := rune(password[i])
		if test < prev {
			return false
		}

		if test == prev {
			if test == match {
				rc = false
			} else if match == '0' || rc == false {
				rc = true
				match = test
			}
		}

		prev = rune(password[i])
	}

	return rc
}

func count_valid(start int, end int, val_func func(string) bool) int {
	count := 0
	for i := start; i <= end; i++ {
		password := strconv.Itoa(i)
		if val_func(password) {
			count++
		}
	}

	return count
}

func dowork(min int, max int, amt int, parallel int, val_func func(string) bool) int {
	var results = make([]chan int, parallel)
	var wg sync.WaitGroup

	for i := 0; i < parallel; i++ {
		results[i] = make(chan int)

		wg.Add(1)

		go func(i int, wg *sync.WaitGroup, min int) {
			defer wg.Done()

			end := min+amt
			if max - end < parallel {
				end = max
			}
			results[i] <- count_valid(min, end, val_func)
		}(i,&wg,min)

		min += amt + 1
	}

	ret := 0

	for i := 0; i < parallel; i++ {
		val := <-results[i]
		ret += val
	}

	wg.Wait()

	return ret
}

func Day4(data string, parallel int) (int, int) {
	s := strings.Split(data, "-")

	if parallel > 65535 {
		parallel = 65535
	}

	min, _ := strconv.Atoi(s[0])
	max, _ := strconv.Atoi(s[1])
	amt := (max - min) / parallel

	if amt <= 0 {
		amt = 1
		parallel = max - min
	}

	pt1 := dowork(min, max, amt, parallel, validate)
	pt2 := dowork(min, max, amt, parallel, validate_extra)

	return pt1, pt2
}