#!/usr/bin/python3

import fileinput
import sys

horizontal = 0
depth = 0
aim = 0
for cmd in (line.rstrip().split(" ", 2) for line in fileinput.input()):
    if cmd[0] == "forward":
        horizontal += int(cmd[1])
        depth += aim * int(cmd[1])
    elif cmd[0] == "down":
        aim += int(cmd[1])
    elif cmd[0] == "up":
        aim -= int(cmd[1])
    else:
        sys.exit( "Unknown command: " + cmd[0] )

print( horizontal * depth )
