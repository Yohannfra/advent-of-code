#!/usr/bin/env python3

import sys
from typing import List
from dataclasses import dataclass


@dataclass
class Octopus:
    energy: int = 0
    flashed: bool = False


def parse_file(fp: str):
    with open(fp, 'r') as f:
        fc = [l.strip() for l in f.readlines()]
        return fc


def str_to_octopus_array(fc: List[str]):
    new = []

    for l in fc:
        li = []
        for c in l:
            li.append(Octopus(int(c)))
        new.append(li)
    return new


def print_arr(arr: List[List[Octopus]]):
    for l in arr:
        for c in l:
            print(c.energy if c.energy >
                  0 else '\033[93m'+str(c.energy)+'\033[0m', end="")
        print("")


def flash(arr: List[List[Octopus]], i: int, j: int):
    arr[i][j].flashed = True
    arr[i][j].energy = 0

    for t in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
        try:  # YOLO
            if arr[i+t[0]][j+t[1]].flashed == False:
                arr[i+t[0]][j+t[1]].energy += 1
            if arr[i+t[0]][j+t[1]].flashed == False and arr[i+t[0]][j+t[1]].energy > 9:
                flash(arr, i+t[0], j+t[1])
        except:
            continue


def do_step(arr: List[List[Octopus]]):
    for i in range(len(arr)):
        for j in range(len(arr)):
            arr[i][j].flashed = False

    for i in range(len(arr)):
        for j in range(len(arr)):
            arr[i][j].energy += 1
            if arr[i][j].energy > 9 and arr[i][j].flashed == False:
                flash(arr, i, j)

    # for i in range(len(arr)):
    #     for j in range(len(arr)):
    #         if arr[i][j].energy > 9 and arr[i][j].flashed == False:
    #             flash(arr, i, j)


def main(argc, argv):
    if argc != 2:
        sys.exit("USAGE: ./day11 input.txt")

    arr = str_to_octopus_array(parse_file(argv[1]))

    print_arr(arr)
    for _ in range(2):
        print("\n")
        do_step(arr)
        print_arr(arr)


if __name__ == '__main__':
    main(len(sys.argv), sys.argv)
