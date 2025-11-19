// [Day 23: LAN Party](https://adventofcode.com/2024/day/23)

package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strings"
	"time"
)

type puzzle struct {
	connections [][2]string
}

func newPuzzle(data string) *puzzle {
	var conns [][2]string
	for _, line := range strings.Split(data, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		if parts := strings.SplitN(line, "-", 2); len(parts) == 2 {
			conns = append(conns, [2]string{parts[0], parts[1]})
		}
	}
	return &puzzle{connections: conns}
}

func (p *puzzle) part1() int {
	adj := make(map[string]map[string]bool)
	for _, e := range p.connections {
		a, b := e[0], e[1]
		if _, ok := adj[a]; !ok {
			adj[a] = make(map[string]bool)
		}
		if _, ok := adj[b]; !ok {
			adj[b] = make(map[string]bool)
		}
		adj[a][b] = true
		adj[b][a] = true
	}

	triangles := make(map[string]struct{})
	for node, neighbors := range adj {
		for n1 := range neighbors {
			for n2 := range neighbors {
				if n1 == n2 {
					continue
				}
				if adj[n1][n2] {
					trio := []string{node, n1, n2}
					sort.Strings(trio)
					key := strings.Join(trio, ",")
					triangles[key] = struct{}{}
				}
			}
		}
	}

	cnt := 0
	for key := range triangles {
		parts := strings.Split(key, ",")
		ok := false
		for _, name := range parts {
			if strings.HasPrefix(name, "t") {
				ok = true
				break
			}
		}
		if ok {
			cnt++
		}
	}
	return cnt
}

// part2 finds largest clique using Bron–Kerbosch algorithm.
func (p *puzzle) part2() string {
	// map node name -> index
	nodes := make(map[string]int)
	var names []string
	for _, e := range p.connections {
		for _, v := range e {
			if _, ok := nodes[v]; !ok {
				nodes[v] = len(names)
				names = append(names, v)
			}
		}
	}

	n := len(names)
	// adjacency as slice of maps
	graph := make([]map[int]bool, n)
	for i := 0; i < n; i++ {
		graph[i] = make(map[int]bool)
	}
	for _, e := range p.connections {
		a := nodes[e[0]]
		b := nodes[e[1]]
		graph[a][b] = true
		graph[b][a] = true
	}

	// sets represented as map[int]struct{}
	type IntSet map[int]struct{}
	newSet := func() IntSet { return make(IntSet) }
	copySet := func(s IntSet) IntSet {
		r := newSet()
		for k := range s {
			r[k] = struct{}{}
		}
		return r
	}
	intersect := func(a, b IntSet) IntSet {
		r := newSet()
		for k := range a {
			if _, ok := b[k]; ok {
				r[k] = struct{}{}
			}
		}
		return r
	}

	cliques := make([][]int, 0)

	var bronKerbosch func(r, pset, x IntSet)
	bronKerbosch = func(r, pset, x IntSet) {
		if len(pset) == 0 && len(x) == 0 {
			var clique []int
			for v := range r {
				clique = append(clique, v)
			}
			cliques = append(cliques, clique)
			return
		}

		var pList []int
		for v := range pset {
			pList = append(pList, v)
		}
		for _, v := range pList {
			rNew := copySet(r)
			rNew[v] = struct{}{}

			neigh := newSet()
			for w := range graph[v] {
				neigh[w] = struct{}{}
			}

			pNew := intersect(pset, neigh)
			xNew := intersect(x, neigh)
			bronKerbosch(rNew, pNew, xNew)

			delete(pset, v)
			x[v] = struct{}{}
		}
	}

	r := newSet()
	pset := newSet()
	x := newSet()
	for i := 0; i < n; i++ {
		pset[i] = struct{}{}
	}

	bronKerbosch(r, pset, x)

	var best []int
	for _, c := range cliques {
		if len(c) > len(best) {
			best = c
		}
	}
	if len(best) == 0 {
		return ""
	}

	res := make([]string, 0, len(best))
	for _, idx := range best {
		res = append(res, names[idx])
	}
	sort.Strings(res)
	return strings.Join(res, ",")
}

func solve(data []byte) (int, string) {
	p := newPuzzle(string(data))
	return p.part1(), p.part2()
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
		r1, r2 := solve(data)
		duration := time.Since(start)

		fmt.Println(r1)
		fmt.Println(r2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		r1, r2 := solve(data)
		fmt.Println(r1)
		fmt.Println(r2)
	}
}
