// [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

package main

import (
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

var (
	WEST  = Coord{X: -1, Y: 0}
	EAST  = Coord{X: 1, Y: 0}
	NORTH = Coord{X: 0, Y: -1}
	SOUTH = Coord{X: 0, Y: 1}
)

type Grid struct {
	data   []byte
	width  int
	height int
}

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

func (g Grid) get(x, y int) byte {
	if x < 0 || x >= g.width || y < 0 || y >= g.height {
		return 0
	}
	return g.data[y*g.width+x]
}

type state struct {
	pos Coord
	dir Coord
}

type Puzzle struct {
	grid  Grid
	start Coord
}

func newPuzzle(data string) *Puzzle {
	grid := parseGrid(data)

	var start Coord
	found := false
	for y := 0; y < grid.height; y++ {
		for x := 0; x < grid.width; x++ {
			if grid.get(x, y) == '^' {
				start = Coord{X: x, Y: y}
				found = true
				break
			}
		}
		if found {
			break
		}
	}

	return &Puzzle{grid: grid, start: start}
}

func (p *Puzzle) moveGuard(x, y *int, direction *Coord, obstruction Coord) bool {
	// obstruction with MAX coordinates to signify there is no obstruction
	noObstruction := Coord{X: 1<<31 - 1, Y: 1<<31 - 1}

	switch *direction {
	case EAST:
		if *x == 0 {
			return true
		}
		nextX, nextY := *x-1, *y
		if p.grid.get(nextX, nextY) == '#' || (nextX == obstruction.X && nextY == obstruction.Y && obstruction != noObstruction) {
			*direction = NORTH
		} else {
			*x = nextX
		}

	case WEST:
		if *x == p.grid.width-1 {
			return true
		}
		nextX, nextY := *x+1, *y
		if p.grid.get(nextX, nextY) == '#' || (nextX == obstruction.X && nextY == obstruction.Y && obstruction != noObstruction) {
			*direction = SOUTH
		} else {
			*x = nextX
		}

	case NORTH:
		if *y == 0 {
			return true
		}
		nextX, nextY := *x, *y-1
		if p.grid.get(nextX, nextY) == '#' || (nextX == obstruction.X && nextY == obstruction.Y && obstruction != noObstruction) {
			*direction = WEST
		} else {
			*y = nextY
		}

	case SOUTH:
		if *y == p.grid.height-1 {
			return true
		}
		nextX, nextY := *x, *y+1
		if p.grid.get(nextX, nextY) == '#' || (nextX == obstruction.X && nextY == obstruction.Y && obstruction != noObstruction) {
			*direction = EAST
		} else {
			*y = nextY
		}
	}

	return false
}

func (p *Puzzle) part1() int {
	x, y := p.start.X, p.start.Y
	direction := NORTH
	leave := false

	seen := make(map[Coord]bool)
	obstruction := Coord{X: 1<<31 - 1, Y: 1<<31 - 1}

	for !leave {
		seen[Coord{X: x, Y: y}] = true
		leave = p.moveGuard(&x, &y, &direction, obstruction)
	}

	return len(seen)
}

func (p *Puzzle) part2() int {
	xy := p.start
	direction := NORTH
	leave := false
	obstruction := Coord{X: 1<<31 - 1, Y: 1<<31 - 1}

	normalWalk := make(map[Coord]bool)

	for !leave {
		normalWalk[xy] = true
		leave = p.moveGuard(&xy.X, &xy.Y, &direction, obstruction)
	}

	stuck := 0

	for y := 0; y < p.grid.height; y++ {
		for x := 0; x < p.grid.width; x++ {
			xy := Coord{X: x, Y: y}
			c := p.grid.get(x, y)

			// optimization: if the guard never walks to this position,
			// an obstruction cannot deviate him
			if !normalWalk[xy] {
				continue
			}

			if c == '.' {
				// can choose this position for the obstruction
				obstruction := Coord{X: x, Y: y}

				xy := p.start
				direction := NORTH
				leave := false
				seen := make(map[state]bool)

				for !leave {
					state := state{pos: xy, dir: direction}
					if seen[state] {
						// same pos, same direction : the guard is stuck
						stuck++
						break
					}
					seen[state] = true

					leave = p.moveGuard(&xy.X, &xy.Y, &direction, obstruction)
				}
			}
		}
	}

	return stuck
}

func solve(data []byte) (int, int) {
	input := string(data)
	puzzle := newPuzzle(input)
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
