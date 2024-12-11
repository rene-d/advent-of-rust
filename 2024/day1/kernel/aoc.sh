#!/bin/sh

sudo insmod ./aoc.ko
cat input.txt > /proc/aoc/input
cat /proc/aoc/part1
cat /proc/aoc/part2
sudo rmmod ./aoc.ko