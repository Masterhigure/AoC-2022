depth = 0
with open("output.txt", "r") as f:
    for line in f.read().splitlines():
        if line == "..":
            depth -= 1
        else:
            depth += 1
        print(depth)
