def read_data(filename):
    f = open(filename, 'r')
    lines = list(map(lambda s: tuple(s.strip().split(" ")), f.readlines()))
    return lines


def solve1(data):
    pos = [0, 0]

    for command in data:
        if command[0] == "forward":
            pos[0] += int(command[1])
        elif command[0] == "down":
            pos[1] += int(command[1])
        elif command[0] == "up":
            pos[1] -= int(command[1])

    return pos[0] * pos[1]


def solve2(data):
    pos = [0, 0, 0]

    for command in data:
        if command[0] == "forward":
            n = int(command[1])
            pos[0] += n
            pos[1] += n * pos[2]
        elif command[0] == "down":
            pos[2] += int(command[1])
        elif command[0] == "up":
            pos[2] -= int(command[1])

    return pos[0] * pos[1]


if __name__ == '__main__':
    lines = read_data("data/2_1")
    # print(solve1(lines))
    print(solve2(lines))
