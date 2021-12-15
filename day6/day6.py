#!/usr/bin/env python3

data = open("input.txt").read().splitlines()

timers = list(map(int, data[0].split(",")))

# step 1 (naive)
for k in range(80):
    n = len(timers)
    for i in range(n):
        timer = timers[i] - 1
        if timer == -1:
            timer = 6
            timers.append(8)
        timers[i] = timer
print(len(timers))

# step 2
timers = [0] * 9
for timer in list(map(int, data[0].split(","))):
    timers[timer] += 1
for day in range(256):
    new = [0] * 9
    for i, timer in enumerate(timers):
        if i == 0:
            new[6] += timer
            new[8] += timer
        else:
            new[i - 1] += timer
    timers = new
print(sum(timers))
