// [Day 12: Garden Groups](https://adventofcode.com/2024/day/12)

package main

import (
	"container/list"
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

// Coord represents a 2D coordinate.
type Coord struct {
	X, Y int
}

// Direction represents a direction.
type Direction int

const (
	North Direction = iota
	East
	South
	West
)

var (
	NORTH = Coord{X: 0, Y: -1}
	EAST  = Coord{X: 1, Y: 0}
	SOUTH = Coord{X: 0, Y: 1}
	WEST  = Coord{X: -1, Y: 0}
)

// Grid represents a 2D grid.
type Grid struct {
	data   []byte
	width  int
	height int
}

// parseGrid parses a grid from input string.
func parseGrid(input string) Grid {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	if len(lines) == 0 {
		return Grid{}
	}

	width := 0
	for _, line := range lines {
		if len(line) > width {
			width = len(line)
		}
	}
	height := len(lines)

	var gridData []byte
	for _, line := range lines {
		gridData = append(gridData, []byte(line)...)
		for len(line) < width {
			gridData = append(gridData, ' ')
			line += " "
		}
	}

	return Grid{
		data:   gridData,
		width:  width,
		height: height,
	}
}

// get returns the value at position (x, y), or 0 if out of bounds.
func (g Grid) get(x, y int) byte {
	if x < 0 || x >= g.width || y < 0 || y >= g.height {
		return 0
	}
	return g.data[y*g.width+x]
}

// iter iterates over all cells in the grid.
func (g Grid) iter() []struct {
	xy   Coord
	cell byte
} {
	var result []struct {
		xy   Coord
		cell byte
	}
	for y := 0; y < g.height; y++ {
		for x := 0; x < g.width; x++ {
			result = append(result, struct {
				xy   Coord
				cell byte
			}{Coord{X: x, Y: y}, g.get(x, y)})
		}
	}
	return result
}

// iterDirectionsAll returns all four directions, including those out of bounds.
func (g Grid) iterDirectionsAll(xy Coord) []struct {
	dir Direction
	pos *Coord
} {
	var result []struct {
		dir Direction
		pos *Coord
	}

	// North
	if xy.Y > 0 {
		pos := Coord{X: xy.X, Y: xy.Y - 1}
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{North, &pos})
	} else {
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{North, nil})
	}

	// East
	if xy.X < g.width-1 {
		pos := Coord{X: xy.X + 1, Y: xy.Y}
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{East, &pos})
	} else {
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{East, nil})
	}

	// South
	if xy.Y < g.height-1 {
		pos := Coord{X: xy.X, Y: xy.Y + 1}
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{South, &pos})
	} else {
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{South, nil})
	}

	// West
	if xy.X > 0 {
		pos := Coord{X: xy.X - 1, Y: xy.Y}
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{West, &pos})
	} else {
		result = append(result, struct {
			dir Direction
			pos *Coord
		}{West, nil})
	}

	return result
}

// iterDirections returns only valid directions (within bounds).
func (g Grid) iterDirections(xy Coord) []Coord {
	var result []Coord

	if xy.Y > 0 {
		result = append(result, Coord{X: xy.X, Y: xy.Y - 1})
	}
	if xy.X < g.width-1 {
		result = append(result, Coord{X: xy.X + 1, Y: xy.Y})
	}
	if xy.Y < g.height-1 {
		result = append(result, Coord{X: xy.X, Y: xy.Y + 1})
	}
	if xy.X > 0 {
		result = append(result, Coord{X: xy.X - 1, Y: xy.Y})
	}

	return result
}

func solve(data []byte) (uint32, uint32) {
	grid := parseGrid(string(data))

	var standardPrice uint32 = 0
	var discountPrice uint32 = 0

	seen := make(map[Coord]bool)

	for _, item := range grid.iter() {
		xy := item.xy
		plant := item.cell

		if seen[xy] {
			continue
		}

		var area uint32 = 0
		var perimeter uint32 = 0
		var sides uint32 = 0
		queue := list.New()
		sideFences := make(map[Direction]map[Coord]bool)

		queue.PushBack(xy)

		for queue.Len() > 0 {
			front := queue.Front()
			queue.Remove(front)
			c := front.Value.(Coord)

			if seen[c] {
				continue
			}
			seen[c] = true

			area++

			for _, dirInfo := range grid.iterDirectionsAll(c) {
				neigh := dirInfo.pos
				if neigh != nil && grid.get(neigh.X, neigh.Y) == plant {
					// BFS to compute area of current plant
					queue.PushBack(*neigh)
					continue
				}

				// fence: increase perimeter
				perimeter++

				// (part 2)
				if sideFences[dirInfo.dir] == nil {
					sideFences[dirInfo.dir] = make(map[Coord]bool)
				}
				sideFences[dirInfo.dir][c] = true
			}
		}

		// Count sides for part 2
		for _, vs := range sideFences {
			seenSides := make(map[Coord]bool)

			for p := range vs {
				if seenSides[p] {
					continue
				}

				sides++

				queueSides := list.New()
				queueSides.PushBack(p)

				for queueSides.Len() > 0 {
					front := queueSides.Front()
					queueSides.Remove(front)
					c := front.Value.(Coord)

					if seenSides[c] {
						continue
					}
					seenSides[c] = true

					for _, a := range grid.iterDirections(c) {
						if vs[a] {
							queueSides.PushBack(a)
						}
					}
				}
			}
		}

		standardPrice += area * perimeter
		discountPrice += area * sides
	}

	return standardPrice, discountPrice
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
