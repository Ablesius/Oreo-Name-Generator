#! /usr/bin/env python3

# WARNING: running this script WILL override the input file!
# Execute with great care!

with open("data/oreo_names.txt") as f:
    for line in f:
        lines = [line.rstrip() for line in f]

names = sorted([line for line in lines])

with open("data/oreo_names.txt", 'w') as f:
    for line in names:
        f.write(line + '\n')
