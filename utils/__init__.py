import random


def random_nums(start=0, end=10, count=10):
    return [random.randint(start, end) for _ in range(count)]


if __name__ == "__main__":
    print(random_nums())
