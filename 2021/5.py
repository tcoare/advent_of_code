#! /home/hardcoare/.pyenv/shims/python3
from read_lines import read_lines
from collections import Counter

def map_vent(coords, x1, y1, x2, y2, diagonal: bool=False):
        if x1==x2 or y1==y2:
            for x in range(min(x1,x2),max(x1,x2)+1):
                for y in range(min(y1,y2),max(y1,y2)+1):
                    coords.append(f"{x},{y}")
            return
        # struggled with p2, used reddit megathread for help
        if diagonal:
            if x1 < x2:
                for x in range(x1,x2+1):
                    if y1<y2:
                        coords.append(f"{x},{x-x1+y1}")
                    else:
                        coords.append(f"{x},{y1-(x-x1)}")
            else:
                for x in range(x2,x1+1):
                    if y2<y1:
                        coords.append(f"{x},{x-x2+y2}")
                    else:
                        coords.append(f"{x},{y2-(x-x2)}")

if __name__ == "__main__":
    path = "input/5.txt"
    lines = read_lines(path)
    coords = list()
    for line in lines:
        x1=int(line.split()[0].split(",")[0])
        y1=int(line.split()[0].split(",")[1])
        x2=int(line.split()[2].split(",")[0])
        y2=int(line.split()[2].split(",")[1])
        map_vent(coords, x1, y1, x2, y2)
    print(len([val for _, val in Counter(coords).items() if val > 1]))

    coords_with_diag = list()
    for line in lines:
        x1=int(line.split()[0].split(",")[0])
        y1=int(line.split()[0].split(",")[1])
        x2=int(line.split()[2].split(",")[0])
        y2=int(line.split()[2].split(",")[1])
        map_vent(coords_with_diag, x1, y1, x2, y2, diagonal=True)
    print(len([val for _, val in Counter(coords_with_diag).items() if val > 1]))
