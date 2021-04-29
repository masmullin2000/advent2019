package main

import (
	//"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

// Max returns the larger of x or y.
func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

// Min returns the smaller of x or y.
func min(x, y int) int {
	if x > y {
		return y
	}
	return x
}

// Abs returns the absolute value of x.
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

type point struct {
	x int
	y int
}

func (p point) manhattan_dist() int {
	return Abs(p.x) + Abs(p.y)
}

type wirepath struct {
	a            point
	b            point
	prev_latency int
}

func (w wirepath) is_hor() bool {
	if w.a.y == w.b.y {
		return true
	}

	return false
}

func (w wirepath) north() int {
	return max(w.a.y, w.b.y)
}

func (w wirepath) south() int {
	return min(w.a.y, w.b.y)
}

func (w wirepath) west() int {
	return min(w.a.x, w.b.x)
}

func (w wirepath) east() int {
	return max(w.a.x, w.b.x)
}

func (w wirepath) get_latency() int {
	if w.is_hor() {
		return w.prev_latency + Abs(w.west()-w.east())
	}
	return w.prev_latency + Abs(w.north()-w.south())
}

func (self wirepath) crosses(wp wirepath) (bool, point) {
	if self.is_hor() == wp.is_hor() {
		return false, point{0, 0}
	}

	if (self.a.x == 0 && self.a.y == 0) ||
		(wp.a.x == 0 && wp.a.y == 0) {
		return false, point{0, 0}
	}

	if self.is_hor() {
		if wp.north() >= self.north() &&
			wp.south() <= self.south() &&
			wp.east() <= self.east() &&
			wp.west() >= self.west() {
			p := point{wp.a.x, self.a.y}
			return true, p
		}
	} else {
		if self.north() >= wp.north() &&
			self.south() <= wp.south() &&
			self.east() <= wp.east() &&
			self.west() >= wp.west() {
			p := point{self.a.x, wp.a.y}
			return true, p
		}
	}

	return false, point{0, 0}
}

func new_wirepath(a point, path string) (point, wirepath) {
	var p2 point
	var wp wirepath

	val, err := strconv.Atoi(path[1:])
	if err != nil {
		panic("couldn't convert path to val")
	}

	switch dir := path[0]; dir {
	case 'R':
		p2 = point{a.x + val, a.y}
	case 'L':
		p2 = point{a.x - val, a.y}
	case 'U':
		p2 = point{a.x, a.y + val}
	case 'D':
		p2 = point{a.x, a.y - val}
	}

	wp = wirepath{a, p2, 0}
	return p2, wp
}

func no_op(wps []wirepath, pd int, cwp wirepath) int {
	return 0
}

func make_wirepath_set(
	paths []string,
	dist_calc func([]wirepath, int, wirepath) int,
	wps1 []wirepath) ([]wirepath, int) {

	p1 := point{0, 0}
	var wps []wirepath
	lat := 0
	dist := math.MaxInt32

	for _, path := range paths {
		p2, wp := new_wirepath(p1, path)
		wp.prev_latency = lat
		lat = wp.get_latency()
		dist = dist_calc(wps1, dist, wp)
		p1 = p2

		wps = append(wps, wp)
	}

	return wps, dist
}

func manhattan_cross_dist(wps []wirepath, pd int, cwp wirepath) int {
	dist := pd
	for _, wp := range wps {
		does_cross, p := wp.crosses(cwp)
		if does_cross {
			pot_dist := p.manhattan_dist()
			if pot_dist < dist {
				dist = pot_dist
			}
		}
	}
	return dist
}

func get_manhattan(paths1 []string, paths2 []string) int {
	wps, _ := make_wirepath_set(paths1, no_op, nil)
	_, dist := make_wirepath_set(paths2, manhattan_cross_dist, wps)

	return dist
}

func latency_cross_dist(wps []wirepath, pd int, cwp wirepath) int {
	dist := pd
	for _, wp := range wps {
		does_cross, p := wp.crosses(cwp)
		if does_cross {
			latency := wp.prev_latency + cwp.prev_latency
			if wp.is_hor() {
				latency += Abs(wp.a.x - p.x)
				latency += Abs(cwp.a.y - p.y)
			} else {
				latency += Abs(wp.a.y - p.y)
				latency += Abs(cwp.a.x - p.x)
			}

			if latency < dist {
				dist = latency
			}
		}
	}
	return dist
}

func get_latency(paths1 []string, paths2 []string) int {
	wps, _ := make_wirepath_set(paths1, no_op, nil)
	_, dist := make_wirepath_set(paths2, latency_cross_dist, wps)

	return dist
}

func process_data(data string) ([]string, []string) {
	lines := strings.Split(data, "\n")

	paths1 := strings.Split(lines[0], ",")
	paths2 := strings.Split(lines[1], ",")

	return paths1, paths2
}

func process_file(file string) string {
	f_data, err := ioutil.ReadFile(file)
	if err != nil {
		panic("didn't read file")
	}

	return string(f_data)
}

func Day3(file string) (int, int) {
	f_data := process_file(file)
	paths1, paths2 := process_data(f_data)

	manhattan := get_manhattan(paths1, paths2)
	latency := get_latency(paths1, paths2)

	return manhattan, latency
}
