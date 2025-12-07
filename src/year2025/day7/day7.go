// [Day 7: Laboratories](https://adventofcode.com/2025/day/7)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

// Coord represents a position in the grid.
type Coord struct {
	X, Y int
}

// Grid is a simple 2D grid of bytes.
type Grid struct {
	data   [][]byte
	width  int
	height int
}

// Parse creates a grid from multiline input.
func Parse(input []byte) *Grid {

	var data [][]byte
	start := 0
	height := 0
	width := 0

	for i, b := range input {
		if b == '\n' {
			if i-start > width {
				width = i - start
			}
			height += 1
			data = append(data, input[start:i])
			start = i + 1
		}
	}

	return &Grid{
		data:   data,
		width:  width,
		height: height,
	}
}

// At returns the cell value at (x, y).
func (g *Grid) At(x, y int) byte {
	return g.data[y][x]
}

// Width returns grid width.
func (g *Grid) Width() int {
	return g.width
}

// Height returns grid height.
func (g *Grid) Height() int {
	return g.height
}

const (
	START    = byte('S')
	SPLITTER = byte('^')
)

// puzzle is the input data.
type puzzle struct {
	grid  *Grid // grid is the diagram of the tachyon manifold.
	start Coord // start is the position where the tachyon beam enters the manifold.
}

// newPuzzle reads the manifold diagram and start position from a multiline input.
func newPuzzle(data []byte) *puzzle {
	grid := Parse(data)

	var start Coord
	found := false

	for y := 0; y < grid.Height(); y++ {
		for x := 0; x < grid.Width(); x++ {
			if grid.At(x, y) == START {
				start = Coord{X: x, Y: y}
				found = true
				break
			}
		}
		if found {
			break
		}
	}

	return &puzzle{grid: grid, start: start}
}

// part1 counts times the beam will be split.
func (p *puzzle) part1() uint64 {
	var splits uint64

	beams := map[int]struct{}{}
	beams[p.start.X] = struct{}{}

	for y := p.start.Y; y < p.grid.Height(); y++ {
		nextBeams := map[int]struct{}{}

		for x := range beams {
			if p.grid.At(x, y) == SPLITTER {
				splits++

				if x > 0 {
					nextBeams[x-1] = struct{}{}
				}
				if x < p.grid.Width()-1 {
					nextBeams[x+1] = struct{}{}
				}
			} else {
				nextBeams[x] = struct{}{}
			}
		}

		beams = nextBeams
	}

	return splits
}

// part2 count the timelines for a single tachyon particle.
func (p *puzzle) part2() uint64 {

	timelines := map[int]uint64{}
	timelines[p.start.X] = 1

	for y := p.start.Y; y < p.grid.Height(); y++ {
		nextTimelines := map[int]uint64{}

		for x, ways := range timelines {
			if p.grid.At(x, y) == SPLITTER {
				if x > 0 {
					nextTimelines[x-1] += ways
				}
				if x < p.grid.Width()-1 {
					nextTimelines[x+1] += ways
				}
			} else {
				nextTimelines[x] += ways
			}
		}

		timelines = nextTimelines
	}

	var total uint64
	for _, v := range timelines {
		total += v
	}

	return total
}

// solve solves parts one and two of the puzzle.
func solve(data []byte) (uint64, uint64) {

	puzzle := newPuzzle(data)

	return puzzle.part1(), puzzle.part2()
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
