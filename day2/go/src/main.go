package main

import (
	"fmt"
)

func main() {
	val,n,v := Day2("../input")
	if n == -1 || v == -1 {
		panic("couldn't find noun verb")
	}
	fmt.Println("Value at position 0", val)
	fmt.Println("noun verb:",n, v)
}