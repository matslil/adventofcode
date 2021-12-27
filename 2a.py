#!/usr/bin/python3

import fileinput

horizontal = 0
depth = 0
for cmd in (line.rstrip().split(" ", 2) for line in fileinput.input()):
    if cmd[0] == "forward":
        horizontal += int(cmd[1])
    elif cmd[0] == "down":
        depth += int(cmd[1])
    elif cmd[0] == "up":
        depth -= int(cmd[1])

print( horizontal * depth )
