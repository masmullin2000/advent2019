package main

import (
	//"fmt"
	"strings"
	"strconv"
	"io/ioutil"
	//"os"
)

func fuel_calculation(mass int) int {
	fuel := (mass / 3) - 2
	if fuel < 0 {
		fuel = 0
	} else {
		fuel = fuel + fuel_calculation(fuel)
	}
	return fuel
}

func Day1(file string) int {
	fuel := 0

	f_data, err := ioutil.ReadFile(file)
	if err != nil {
		panic("didn't read file")
	}

	lines := strings.Split(string(f_data),"\n")
	for _,l := range lines {
		if l != "" {
			mass, err := strconv.Atoi(l)
			if err != nil {
				return -1
			}
			fuel += fuel_calculation(mass)
		}
	}
	
	return fuel
}
