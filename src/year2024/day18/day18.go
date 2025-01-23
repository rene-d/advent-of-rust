//! [Day 18: RAM Run](https://adventofcode.com/2024/day/18)

package main

import (
	"bufio"
	"fmt"
	"os"
)

type Point struct {
	x, y int
}

type QueueItem struct {
	pos   Point
	steps uint
}

func readInput(filename string) []Point {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var points []Point
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var x, y int
		fmt.Sscanf(scanner.Text(), "%d,%d", &x, &y)
		points = append(points, Point{x, y})
	}
	return points
}

func findPath(memory [][]bool, memSize int) uint {

	start := Point{0, 0}
	end := Point{memSize - 1, memSize - 1}

	if memory[start.y][start.x] || memory[end.y][end.x] {
		return 0
	}

	seen := make([][]bool, len(memory))
	for i := range seen {
		seen[i] = make([]bool, len(memory[0]))
	}

	dirs := []Point{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	queue := []QueueItem{{start, 0}}
	seen[start.y][start.x] = true

	for len(queue) > 0 {
		pos := queue[0]
		queue = queue[1:]

		if pos.pos == end {
			return pos.steps
		}

		for _, dir := range dirs {
			new_pos := Point{pos.pos.x + dir.x, pos.pos.y + dir.y}
			if new_pos.x >= 0 && new_pos.x < memSize && new_pos.y >= 0 && new_pos.y < memSize &&
				!memory[new_pos.y][new_pos.x] && !seen[new_pos.y][new_pos.x] {
				seen[new_pos.y][new_pos.x] = true
				queue = append(queue, QueueItem{new_pos, pos.steps + 1})
			}
		}
	}
	return 0
}

func main() {
	inputFile := "input.txt"
	if len(os.Args) >= 2 {
		inputFile = os.Args[1]
	}

	byte_positions := readInput(inputFile)
	memSize := 71
	numCorruptions := 1024

	// Part 1
	memory := make([][]bool, memSize)
	for i := range memory {
		memory[i] = make([]bool, memSize)
	}

	for i := 0; i < numCorruptions && i < len(byte_positions); i++ {
		p := byte_positions[i]
		memory[p.y][p.x] = true
	}

	steps := findPath(memory, memSize)
	fmt.Println(steps)

	// Part 2
	memory = make([][]bool, memSize)
	for i := range memory {
		memory[i] = make([]bool, memSize)
	}

	for _, p := range byte_positions {
		memory[p.y][p.x] = true
		steps := findPath(memory, memSize)
		if steps == 0 {
			fmt.Printf("%d,%d\n", p.x, p.y)
			break
		}
	}
}
