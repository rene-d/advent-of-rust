// [Day 24: Crossed Wires](https://adventofcode.com/2024/day/24)

package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

// Role represents the functional role of a wire in the adder circuit analysis.
type Role int

const (
	CarryOut Role = iota
	IntXorXor
	ABAndGate
	AndGateWires
	SumOut
)

// Operation is the type of boolean operation implemented by a gate.
type Operation int

const (
	OpAnd Operation = iota
	OpOr
	OpXor
)

// OperationFrom converts the textual gate operator ("AND", "OR", "XOR")
// into the corresponding Operation value.
func OperationFrom(s string) Operation {
	switch s {
	case "AND":
		return OpAnd
	case "OR":
		return OpOr
	case "XOR":
		return OpXor
	default:
		panic("unknown op: " + s)
	}
}

// Eval computes the result of the operation on the two 1-bit inputs a and b.
func (o Operation) Eval(a, b uint8) uint8 {
	switch o {
	case OpAnd:
		return a & b
	case OpOr:
		return a | b
	case OpXor:
		return a ^ b
	}
	return 0
}

// Gate describes a logic gate: inputs A and B, operator Op, and result wire R.
type Gate struct {
	A  string
	B  string
	Op Operation
	R  string
}

// Puzzle holds the parsed circuit: initial wire values and the list of gates.
type Puzzle struct {
	wires map[string]uint8
	gates []Gate
}

// NewPuzzle parses the input text and returns a Puzzle containing
// initial wire assignments and the parsed gates.
func NewPuzzle(data string) *Puzzle {
	wires := make(map[string]uint8)
	var gates []Gate

	for _, line := range strings.Split(data, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if strings.Contains(line, ": ") {
			parts := strings.SplitN(line, ": ", 2)
			v, _ := strconv.ParseUint(strings.TrimSpace(parts[1]), 10, 8)
			wires[strings.TrimSpace(parts[0])] = uint8(v)
		}

		if strings.Contains(line, " -> ") {
			f := strings.Fields(line)
			if len(f) >= 5 {
				g := Gate{
					A:  f[0],
					Op: OperationFrom(f[1]),
					B:  f[2],
					R:  f[4],
				}
				gates = append(gates, g)
			}
		}
	}

	return &Puzzle{wires: wires, gates: gates}
}

// copyWires returns a shallow copy of the provided wire map.
func copyWires(src map[string]uint8) map[string]uint8 {
	dst := make(map[string]uint8, len(src))
	for k, v := range src {
		dst[k] = v
	}
	return dst
}

// Part1 simulates the logic circuit until all gates are evaluated and
// returns a bitmask built from wires named "zN" that have value 1.
func (p *Puzzle) Part1() uint64 {
	waiting := make([]*Gate, 0, len(p.gates))
	for i := range p.gates {
		waiting = append(waiting, &p.gates[i])
	}

	wires := copyWires(p.wires)

	for len(waiting) > 0 {
		var nextWaiting []*Gate

		for _, gate := range waiting {
			a, oka := wires[gate.A]
			if oka {
				b, okb := wires[gate.B]
				if okb {
					r := gate.Op.Eval(a, b)
					wires[gate.R] = r
					continue
				}
			}
			nextWaiting = append(nextWaiting, gate)
		}

		waiting = nextWaiting
	}

	var acc uint64
	for k, v := range wires {
		if strings.HasPrefix(k, "z") && v == 1 {
			idx, err := strconv.ParseUint(k[1:], 10, 64)
			if err == nil {
				acc |= (1 << idx)
			}
		}
	}

	return acc
}

// RoleSet is a small set implementation for Role values used during analysis.
type RoleSet map[Role]struct{}

// newRoleSet creates an empty RoleSet.
func newRoleSet() RoleSet { return make(RoleSet) }

// insert adds a role to the set.
func (s RoleSet) insert(r Role) { s[r] = struct{}{} }

// len returns the size of the set.
func (s RoleSet) len() int { return len(s) }

// contains reports whether the set contains r.
func (s RoleSet) contains(r Role) bool {
	_, ok := s[r]
	return ok
}

// isRole reports true when the set contains exactly the role r.
func isRole(s RoleSet, r Role) bool {
	return s.len() == 1 && s.contains(r)
}

// isRoles reports true when the set contains exactly the two roles r1 and r2.
func isRoles(s RoleSet, r1, r2 Role) bool {
	if s.len() != 2 {
		return false
	}
	return s.contains(r1) && s.contains(r2)
}

// Part2 analyses the wiring of the adder circuit and returns a
// comma-separated list of wires that are considered "bad" (incorrectly wired).
func (p *Puzzle) Part2() string {
	inputTypes := make(map[string]RoleSet)
	resultTypes := make(map[string]RoleSet)

	for _, gate := range p.gates {
		if (gate.A == "x00" && gate.B == "y00") || (gate.B == "x00" && gate.A == "y00") {
			continue
		}

		addResultRole := func(r Role) {
			if _, ok := resultTypes[gate.R]; !ok {
				resultTypes[gate.R] = newRoleSet()
			}
			resultTypes[gate.R].insert(r)
		}

		if (strings.HasPrefix(gate.A, "x") && strings.HasPrefix(gate.B, "y")) || (strings.HasPrefix(gate.A, "y") && strings.HasPrefix(gate.B, "x")) {
			switch gate.Op {
			case OpXor:
				addResultRole(IntXorXor)
			case OpAnd:
				addResultRole(ABAndGate)
			case OpOr:
				panic("OR gate should be wired to x/y")
			}
		} else {
			var role Role
			switch gate.Op {
			case OpXor:
				role = SumOut
			case OpAnd:
				role = AndGateWires
			case OpOr:
				role = CarryOut
			}

			if _, ok := inputTypes[gate.A]; !ok {
				inputTypes[gate.A] = newRoleSet()
			}
			if _, ok := inputTypes[gate.B]; !ok {
				inputTypes[gate.B] = newRoleSet()
			}
			inputTypes[gate.A].insert(role)
			inputTypes[gate.B].insert(role)
			addResultRole(role)
		}
	}

	// find last z wire by numeric suffix
	var lastZ string
	var lastIdx int = -1
	for k := range resultTypes {
		if strings.HasPrefix(k, "z") {
			if idx, err := strconv.Atoi(k[1:]); err == nil {
				if idx > lastIdx {
					lastIdx = idx
					lastZ = k
				}
			}
		}
	}

	var badWires []string

	for wire, res := range resultTypes {
		inp, _ := inputTypes[wire]

		if wire == lastZ && isRole(res, CarryOut) {
			continue
		}

		if (inp == nil || inp.len() == 0) && strings.HasPrefix(wire, "z") && isRole(res, SumOut) {
			continue
		}

		if isRole(inp, CarryOut) && (isRole(res, AndGateWires) || isRole(res, ABAndGate)) {
			continue
		}

		if isRoles(inp, SumOut, AndGateWires) && (isRole(res, CarryOut) || isRole(res, IntXorXor)) {
			continue
		}

		badWires = append(badWires, wire)
	}

	sort.Strings(badWires)
	return strings.Join(badWires, ",")
}

// solve is the package-level solver used by main and tests. It accepts raw
// file bytes and returns the two results (part1 as uint64 and part2 as string).
func solve(data []byte) (uint64, string) {
	p := NewPuzzle(string(data))
	return p.Part1(), p.Part2()
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
