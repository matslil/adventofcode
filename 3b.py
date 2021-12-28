#!/usr/bin/python3

import fileinput
from typing import List

def filter_oxygen( input:List[str], bit:int ) -> List[str]:
    matches = 0
    output = []
    for line in input:
        if line[bit:bit+1] == '1':
            matches += 1
    if matches >= ( len(input)/2 ):
        new_match = '1'
    else:
        new_match = '0'
    for line in input:
        if line[bit:bit+1] == new_match:
            output.append(line)
    return output

def filter_co2( input:List[str], bit:int ) -> List[str]:
    matches = 0
    output = []
    for line in input:
        if line[bit:bit+1] == '1':
            matches += 1
    if matches >= ( len(input)/2 ):
        new_match = '0'
    else:
        new_match = '1'
    for line in input:
        if line[bit:bit+1] == new_match:
            output.append(line)
    return output

my_input = [ line.rstrip() for line in fileinput.input() ]
oxygen = my_input
co2 = my_input

for bit in range(len(my_input[0])):
    if len(oxygen) > 1:
        oxygen = filter_oxygen( oxygen, bit )
    if len(co2) > 1:
        co2 = filter_co2( co2, bit )

print( oxygen )
print( co2 )
print( int(oxygen[0], 2) * int(co2[0], 2) )
