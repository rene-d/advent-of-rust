// [Day 10: Hoof It](https://adventofcode.com/2024/day/10)

package main

import (
	"container/list"
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

const (
	bottom = byte('0')
	top    = byte('9')
)

// Point represents a 2D coordinate.
type Point struct {
	X, Y int
}

type Grid struct {
	data   []byte
	width  int
	height int
}

func parseGrid(data string) Grid {
	lines := strings.Split(strings.TrimSpace(data), "\n")
	height := len(lines)
	width := len(lines[0])

	var gridData []byte
	for _, line := range lines {
		gridData = append(gridData, []byte(line)...)
	}

	return Grid{
		data:   gridData,
		width:  width,
		height: height,
	}
}

func (g Grid) get(p Point) byte {
	if p.X < 0 || p.X >= g.width || p.Y < 0 || p.Y >= g.height {
		return 0
	}
	return g.data[p.Y*g.width+p.X]
}

func (g Grid) neighbors(p Point) []Point {
	var neighbors []Point
	if p.Y > 0 {
		neighbors = append(neighbors, Point{X: p.X, Y: p.Y - 1}) // North
	}
	if p.X < g.width-1 {
		neighbors = append(neighbors, Point{X: p.X + 1, Y: p.Y}) // East
	}
	if p.Y < g.height-1 {
		neighbors = append(neighbors, Point{X: p.X, Y: p.Y + 1}) // South
	}
	if p.X > 0 {
		neighbors = append(neighbors, Point{X: p.X - 1, Y: p.Y}) // West
	}
	return neighbors
}

type queueItem struct {
	point  Point
	height byte
}

func (g Grid) bfs(start Point) int {
	visited := make(map[Point]bool)
	height9 := make(map[Point]bool)
	queue := list.New()

	queue.PushBack(queueItem{start, bottom})

	for queue.Len() > 0 {
		front := queue.Front()
		queue.Remove(front)
		item := front.Value.(queueItem)

		xy := item.point
		height := item.height

		visited[xy] = true

		if g.get(xy) == top {
			height9[xy] = true
		}

		for _, neigh := range g.neighbors(xy) {
			if g.get(neigh) == height+1 && !visited[neigh] {
				queue.PushBack(queueItem{neigh, height + 1})
			}
		}
	}

	return len(height9)
}

func (g Grid) dfs(xy Point, height byte) int {
	if g.get(xy) == top {
		return 1
	}

	sum := 0
	for _, neigh := range g.neighbors(xy) {
		if g.get(neigh) == height+1 {
			sum += g.dfs(neigh, height+1)
		}
	}
	return sum
}

func part1(data string) int {
	grid := parseGrid(data)
	sum := 0

	for y := 0; y < grid.height; y++ {
		for x := 0; x < grid.width; x++ {
			p := Point{X: x, Y: y}
			if grid.get(p) == bottom {
				sum += grid.bfs(p)
			}
		}
	}

	return sum
}

func part2(data string) int {
	grid := parseGrid(data)
	sum := 0

	for y := 0; y < grid.height; y++ {
		for x := 0; x < grid.width; x++ {
			p := Point{X: x, Y: y}
			if grid.get(p) == bottom {
				sum += grid.dfs(p, bottom)
			}
		}
	}

	return sum
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
