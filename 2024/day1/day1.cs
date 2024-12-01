// [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

using System;
using System.Collections.Generic;
using System.IO;

class day1
{
    static void Main(string[] args)
    {
        // Default to "input.txt" if no argument is provided
        string inputFile = args.Length == 0 ? "input.txt" : args[0];

        // Lists to store the values
        List<int> left = new List<int>();
        List<int> right = new List<int>();

        // Read the file and populate the left and right lists
        foreach (var line in File.ReadLines(inputFile))
        {
            // Split the line by any whitespace (spaces or tabs)
            var parts = line.Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);

            // Add the two integers to left and right lists
            left.Add(int.Parse(parts[0]));
            right.Add(int.Parse(parts[1]));
        }

        // Sort both lists
        left.Sort();
        right.Sort();

        // Part 1: Calculate the sum of absolute differences
        int part1Result = 0;
        for (int i = 0; i < left.Count; i++)
        {
            part1Result += Math.Abs(left[i] - right[i]);
        }
        Console.WriteLine(part1Result);

        // Part 2: Count occurrences of each element in the right list
        Dictionary<int, int> rightCounts = new Dictionary<int, int>();
        foreach (var num in right)
        {
            if (rightCounts.ContainsKey(num))
            {
                rightCounts[num]++;
            }
            else
            {
                rightCounts[num] = 1;
            }
        }

        // Calculate the weighted sum for part 2
        int part2Result = 0;
        foreach (var num in left)
        {
            if (rightCounts.ContainsKey(num))
            {
                part2Result += num * rightCounts[num];
            }
        }
        Console.WriteLine(part2Result);
    }
}
