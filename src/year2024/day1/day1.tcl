#!/usr/bin/env tclsh
# [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

# Usage: ./day1.tcl [input.txt]

if {![info exists env(PATH)]} {
    # noop, ensure tclsh has access to env if needed
}

set filename "input.txt"
if {$argc >= 1} {
    set filename [lindex $argv 0]
}

set left {}
set right {}

set fp [open $filename r]
while {[gets $fp line] >= 0} {
    set line [string trim $line]
    if {$line eq ""} continue
    # parse exactly two integers from the line; skip if not present
    set a 0
    set b 0
    if {[scan $line "%d %d" a b] != 2} {
        continue
    }
    lappend left $a
    lappend right $b
}
close $fp

set left_sorted [lsort -integer $left]
set right_sorted [lsort -integer $right]

# part 1: sum of absolute differences of paired elements
set n [llength $left_sorted]
set sum1 0
for {set i 0} {$i < $n} {incr i} {
    set a [lindex $left_sorted $i]
    set b [lindex $right_sorted $i]
    set diff [expr {abs($a - $b)}]
    incr sum1 $diff
}
puts $sum1

# part 2: sum over left values of (value * count_of_same_value_in_right)
array unset right_counts
foreach b $right_sorted {
    incr right_counts($b)
}

set sum2 0
foreach a $left_sorted {
    if {[info exists right_counts($a)]} {
        set cnt $right_counts($a)
    } else {
        set cnt 0
    }
    incr sum2 [expr {$a * $cnt}]
}
puts $sum2
