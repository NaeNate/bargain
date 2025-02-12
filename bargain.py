import sys

path = sys.argv[1]

if path.split(".")[-1] != "brg":
    print("File not supported")
    sys.exit(0)

with open(path) as file:
    lines = file.read().splitlines()

variables = {}

for line in lines:
    if not line:
        continue

    split = line.split()

    match split[0]:
        case "var":
            value = split[2]

            if value.isdigit():
                value = int(value)
            else:
                value = value[1:-1]

            variables[split[1]] = value
        case "print":
            print(variables[split[1]])
