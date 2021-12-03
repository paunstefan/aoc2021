def read_data(filename):
    f = open(filename, 'r')
    lines = list(map(lambda s: int(s.strip()), f.readlines()))
    return lines


def solve1(data):
    c = 0
    for i in range(1, len(data)):
        if data[i] > data[i - 1]:
            c += 1
    return c


def solve2(data):
    c = 0
    for i in range(1, len(data) - 2):
        if window_sum(data, i) > window_sum(data, i - 1):
            c += 1
    return c


def window_sum(data, index):
    return sum(data[index: index + 3])


if __name__ == '__main__':
    lines = read_data("data/1_1")
    print(solve1(lines))
    print(solve2(lines))
