import sys
from typing import List

def read_lines(fp: str) -> List[str]:
    try:
        with open(fp, 'r') as f:
            lines = [l.strip() for l in f.readlines()]
            return lines
    except:
        print('Error reading lines in ', fp)
        sys.exit(1)
