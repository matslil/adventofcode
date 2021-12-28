#!/usr/bin/python3

import fileinput

bitfield = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
lines = 0

for bits in fileinput.input():
    lines += 1
    idx = 0
    for bit in list(bits):
        if bit == "1":
            bitfield[idx] += 1
        idx += 1

gamma = 0
epsilon = 0
for idx in range(12):
    if bitfield[idx] > ( lines / 2):
        gamma += 2 ** (11 - idx)
    else:
        epsilon += 2 ** (11 -idx)

print( gamma * epsilon )
