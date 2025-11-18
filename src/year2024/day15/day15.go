// [Day 15: Warehouse Woes](https://adventofcode.com/2024/day/15)

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

var (
	RIGHT = Coord{X: 1, Y: 0}
	LEFT  = Coord{X: -1, Y: 0}
	DOWN  = Coord{X: 0, Y: 1}
	UP    = Coord{X: 0, Y: -1}
)

func (c Coord) add(d Coord) Coord {
	return Coord{X: c.X + d.X, Y: c.Y + d.Y}
}

type Grid struct {
	data     []byte
	width    int
	height   int
	exterior byte
}

func parseGrid(input string, exterior byte) Grid {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	if len(lines) == 0 {
		return Grid{exterior: exterior}
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
		data:     gridData,
		width:    width,
		height:   height,
		exterior: exterior,
	}
}

func (g Grid) get(p Coord) byte {
	if p.X < 0 || p.X >= g.width || p.Y < 0 || p.Y >= g.height {
		return g.exterior
	}
	return g.data[p.Y*g.width+p.X]
}

func (g *Grid) set(p Coord, c byte) {
	if p.X < 0 || p.X >= g.width || p.Y < 0 || p.Y >= g.height {
		return
	}
	g.data[p.Y*g.width+p.X] = c
}

func (g Grid) withSize(width, height int, value, exterior byte) Grid {
	data := make([]byte, width*height)
	for i := range data {
		data[i] = value
	}
	return Grid{
		data:     data,
		width:    width,
		height:   height,
		exterior: exterior,
	}
}

func score(grid Grid) int {
	sum := 0
	for y := 0; y < grid.height; y++ {
		for x := 0; x < grid.width; x++ {
			c := grid.get(Coord{X: x, Y: y})
			if c == 'O' || c == '[' {
				sum += 100*y + x
			}
		}
	}
	return sum
}

func initFirstWarehouse(input string) (Grid, Coord) {
	grid := parseGrid(input, '#')
	var start Coord

	for y := 0; y < grid.height; y++ {
		for x := 0; x < grid.width; x++ {
			pos := Coord{X: x, Y: y}
			if grid.get(pos) == '@' {
				start = pos
				grid.set(pos, '.')
				break
			}
		}
		if start.X != 0 || start.Y != 0 {
			break
		}
	}

	return grid, start
}

func initSecondWarehouse(input string) (Grid, Coord) {
	simple := parseGrid(input, '#')
	grid := simple.withSize(simple.width*2, simple.height, ' ', '#')
	var start Coord

	for y := 0; y < simple.height; y++ {
		for x := 0; x < simple.width; x++ {
			pos := Coord{X: x, Y: y}
			c := simple.get(pos)
			pos1 := Coord{X: x * 2, Y: y}
			pos2 := Coord{X: x*2 + 1, Y: y}

			switch c {
			case '@':
				start = pos1
				grid.set(pos1, '.')
				grid.set(pos2, '.')
			case 'O':
				grid.set(pos1, '[')
				grid.set(pos2, ']')
			default:
				grid.set(pos1, c)
				grid.set(pos2, c)
			}
		}
	}

	return grid, start
}

func moveBoxes(grid *Grid, robot *Coord, d Coord) {
	seen := make(map[Coord]bool)
	queue := list.New()
	queue.PushBack(*robot)

	for queue.Len() > 0 {
		front := queue.Front()
		queue.Remove(front)
		pos := front.Value.(Coord)

		if seen[pos] {
			continue
		}
		seen[pos] = true

		newPos := pos.add(d)

		switch grid.get(newPos) {
		case '#':
			return
		case '[':
			queue.PushBack(newPos)
			queue.PushBack(newPos.add(RIGHT))
		case ']':
			queue.PushBack(newPos.add(LEFT))
			queue.PushBack(newPos)
		case '.':
			// continue
		}
	}

	for len(seen) > 0 {
		seenNew := make(map[Coord]bool)

		for pos := range seen {
			newPos := pos.add(d)

			if seen[newPos] {
				seenNew[pos] = true
			} else {
				grid.set(newPos, grid.get(pos))
				grid.set(pos, '.')
			}
		}

		seen = seenNew
	}

	*robot = robot.add(d)
}

func part1(data string) int {
	parts := strings.SplitN(data, "\n\n", 2)
	if len(parts) != 2 {
		return 0
	}
	warehouseData := parts[0]
	movesStr := parts[1]

	grid, robot := initFirstWarehouse(warehouseData)

	for _, m := range movesStr {
		var d Coord
		switch m {
		case '>':
			d = RIGHT
		case '<':
			d = LEFT
		case 'v':
			d = DOWN
		case '^':
			d = UP
		default:
			continue
		}

		newPos := robot.add(d)
		switch grid.get(newPos) {
		case '.':
			robot = newPos
		case 'O':
			i := 1
			for grid.get(robot.add(Coord{X: d.X * i, Y: d.Y * i})) == 'O' {
				i++
			}
			finalPos := robot.add(Coord{X: d.X * i, Y: d.Y * i})
			if grid.get(finalPos) == '.' {
				grid.set(finalPos, 'O')
				grid.set(newPos, '.')
				robot = newPos
			}
		case '#':
			// cannot move
		}
	}

	return score(grid)
}

func part2(data string) int {
	parts := strings.SplitN(data, "\n\n", 2)
	if len(parts) != 2 {
		return 0
	}
	warehouseData := parts[0]
	movesStr := parts[1]

	grid, robot := initSecondWarehouse(warehouseData)

	for _, m := range movesStr {
		var d Coord
		switch m {
		case '>':
			d = RIGHT
		case '<':
			d = LEFT
		case 'v':
			d = DOWN
		case '^':
			d = UP
		default:
			continue
		}

		newPos := robot.add(d)
		switch grid.get(newPos) {
		case '.':
			robot = newPos
		case 'O', '[', ']':
			moveBoxes(&grid, &robot, d)
		}
	}

	return score(grid)
}

func solve(data []byte) (int, int) {
	input := string(data)
	return part1(input), part2(input)
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
