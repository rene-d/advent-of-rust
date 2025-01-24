// [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

// javac day1.java
// java day1 [input.txt]

import java.io.*;
import java.nio.file.*;
import java.util.*;

class day1 {
    public static void main(String[] args) throws IOException {
        // Default to "input.txt" if no arguments are passed
        String inputFile = args.length == 0 ? "input.txt" : args[0];

        // Read the file into two lists: left and right
        List<Integer> left = new ArrayList<>();
        List<Integer> right = new ArrayList<>();

        // Read the file and split the numbers
        Files.lines(Paths.get(inputFile)).forEach(line -> {
            String[] parts = line.split("\\s+");
            left.add(Integer.parseInt(parts[0]));
            right.add(Integer.parseInt(parts[1]));
        });

        // Sort both lists
        Collections.sort(left);
        Collections.sort(right);

        // Part 1: Calculate the sum of absolute differences
        int part1 = 0;
        for (int i = 0; i < left.size(); i++) {
            part1 += Math.abs(left.get(i) - right.get(i));
        }
        System.out.println(part1);

        // Part 2: Count the occurrences of each number in the right list
        Map<Integer, Integer> rightCounts = new HashMap<>();
        for (int num : right) {
            rightCounts.put(num, rightCounts.getOrDefault(num, 0) + 1);
        }

        // Calculate the weighted sum of left elements based on right counts
        int part2 = 0;
        for (int num : left) {
            part2 += num * rightCounts.getOrDefault(num, 0);
        }
        System.out.println(part2);
    }
}
