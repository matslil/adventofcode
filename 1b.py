#!/usr/bin/python3

import fileinput

inc = 0

def three_sums() -> int:
    sums = []
    idx = 0
    for value in (int(line) for line in fileinput.input()):
        if idx >= 3:
            sums[ idx % 3 ] = value
        else:
            sums.append(value)
        if idx >= 2:
            yield sums[0] + sums[1] + sums[2]
        idx += 1

prev:int = -1
for sum in three_sums():
    if prev >= 0 and sum > prev:
        inc += 1
    prev = sum

print( inc )

