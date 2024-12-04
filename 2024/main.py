#!/usr/bin/env python3

import sys

from days import day1
from days import day2
from days import day3


def main():
    if len(sys.argv) != 2:
        print('USAGE: main.py input.txt')
        sys.exit(1)

    # day1.part1(sys.argv[1])
    # day1.part2(sys.argv[1])

    # day2.part1(sys.argv[1])
    # day2.part2(sys.argv[1])

    # day3.part1(sys.argv[1])
    day3.part2(sys.argv[1])

if __name__ == '__main__':
    main()
