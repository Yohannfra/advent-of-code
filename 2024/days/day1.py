from .utils import read_lines

def part1(fp: str):
    lines = read_lines(fp)
    list1 = []
    list2 = []

    for line in lines:
        spl = line.split()
        list1.append(int(spl[0]))
        list2.append(int(spl[1]))

    list1.sort()
    list2.sort()

    sum = 0
    for i in range(len(list1)):
        n1 = list1[i]
        n2 = list2[i]
        if n1 > n2:
            sum += n1 - n2
        else:
            sum += n2 - n1
    print(sum)

def part2(fp: str):
    lines = read_lines(fp)
    list1 = []
    list2 = []

    for line in lines:
        spl = line.split()
        list1.append(int(spl[0]))
        list2.append(int(spl[1]))

    sum = 0
    for n in list1:
        count = list2.count(n)
        sum += count * n

    print(sum)
