#!/usr/bin/python3

import fileinput

inc = 0
prev:int = -1
for arg in (int(line) for line in fileinput.input()):
    if prev >= 0 and arg > prev:
        inc += 1
    prev = arg

print( inc )

