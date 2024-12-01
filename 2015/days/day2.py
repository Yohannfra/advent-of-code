from .utils import read_lines

def part1(fp: str):
    lines = read_lines(fp)

    sum = 0

    for line in lines:
        spl = line.split('x')
        l,w,h = int(spl[0]), int(spl[1]), int(spl[2])

        s1 = l*w
        s2 = w*h
        s3 = h*l

        area = (2*s1) + (2*s2) + (2*s3)

        sum += area + min([s1,s2,s3])

    print(sum)
