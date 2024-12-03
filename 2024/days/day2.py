from typing import List
from .utils import read_lines


def split_int_line(line: str) -> List[int]:
    spl = line.split()
    nbs = [int(n) for n in spl]
    return nbs


def is_increasing(nbs: List[int]) -> bool:
    n1 = nbs[0]
    for n2 in nbs[1:]:
        if n2 <= n1:
            return False

        if n2 - 3 > n1:
            return False

        n1 = n2

    return True


def is_decreasing(nbs: List[int]) -> bool:
    n1 = nbs[0]
    for n2 in nbs[1:]:
        if n2 >= n1:
            return False

        if n2 + 3 < n1:
            return False

        n1 = n2

    return True


def part1(fp: str):
    lines = read_lines(fp)

    safe_count = 0

    for line in lines:
        sp = split_int_line(line)

        if is_increasing(sp) == False and is_decreasing(sp) == False:
            continue

        safe_count += 1

    print(safe_count)


def part2(fp: str):
    lines = read_lines(fp)

    safe_count = 0

    for line in lines:
        sp = split_int_line(line)

        safe = False

        for i in range(len(sp)):
            sp2 = sp[::]
            del sp2[i]

            # print(sp2)

            if is_increasing(sp2) == False and is_decreasing(sp2) == False:
                continue
            safe = True
            break

        if safe == False:
            # print('-----------------------')
            continue

        safe_count += 1

    print(safe_count)
