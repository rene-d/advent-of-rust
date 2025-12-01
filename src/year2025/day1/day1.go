// [Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

type Op struct {
	dir byte
	num int
}

// part1 computes the password to open the door.
func part1(ops []Op) int {

	pos := 50

	countZero := 0

	for _, op := range ops {

		if op.dir == 'L' {
			pos = (pos - op.num + 100) % 100
		} else { // 'R'
			pos = (pos + op.num) % 100
		}

		if pos == 0 {
			countZero++
		}
	}

	return countZero
}

// part2 computes the password to open the door according to password method 0x434C49434B (aka 'CLCK').
func part2(ops []Op) int {

	pos := 50

	countZero := 0

	for _, op := range ops {

		step := 1 // 'R'
		if op.dir == 'L' {
			step = -1
		}

		for i := 0; i < op.num; i++ {
			pos = (pos + step + 100) % 100
			if pos == 0 {
				countZero++
			}
		}
	}

	return countZero
}

// solve solves parts one and two of the puzzle.
func solve(data []byte) (int, int) {

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	ops := make([]Op, 0, len(lines))

	for _, line := range lines {
		num, err := strconv.Atoi(line[1:])
		if err == nil {
			ops = append(ops, Op{dir: line[0], num: num})
		}

	}

	return part1(ops), part2(ops)
}

func main() {
	filename := "input.txt"
	elapsed := false

	for i := 1; i < len(os.Args); i++ {
		arg := os.Args[i]
		if arg == "--elapsed" {
			elapsed = true
		} else if !strings.HasPrefix(arg, "-") {
			filename = arg
		}
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	if elapsed {
		start := time.Now()
		result1, result2 := solve(data)
		duration := time.Since(start)

		fmt.Println(result1)
		fmt.Println(result2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		result1, result2 := solve(data)

		fmt.Println(result1)
		fmt.Println(result2)
	}
}
