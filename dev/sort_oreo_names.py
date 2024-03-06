#! /usr/bin/env python3

# WARNING: running this script WILL override the input file!
# Execute with great care!

with open("src/data/oreo_names.txt") as f:
    lines = [line.rstrip() for line in f]

names = sorted([line for line in lines])

with open("src/data/oreo_names.txt", 'w') as f:
    for line in names:
        f.write(line + '\n')
