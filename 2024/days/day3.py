from .utils import read_lines
import re


def part1(fp: str):
    lines = read_lines(fp)

    sum = 0

    for l in lines:
        matches = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", l)

        for m in matches:
            sum += int(m[0]) * int(m[1])

    print(sum)


def part2(fp: str):
    lines = read_lines(fp)

    enabled = True
    sum = 0

    for l in lines:
        matches = re.findall(
            r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))", l)

        for m in matches:
            if m[0] == 'do()':
                enabled = True
            elif m[0] == "don't()":
                enabled = False
            else:
                if enabled:
                    sum += int(m[1]) * int(m[2])

    print(sum)
