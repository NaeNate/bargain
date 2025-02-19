import sys

path = sys.argv[1]

if path.split(".")[-1] != "brg":
    print("File not supported")
    sys.exit(0)

with open(path) as file:
    lines = file.read().splitlines()

blocks = []
current = blocks
indentation = 0

for line in lines:
    strip = line.strip()

    if not strip:
        continue

    i = len(line) - len(strip)

    if i > indentation:
        blocks[-1].append(strip)
    else:
        blocks.append([strip])

    indentation = i


print(blocks)

# variables = {}


# def evaluate(value):
#     if value.isdigit() or value.startswith('"'):
#         return value
#     else:
#         return variables[value]


# for line in lines:
#     if not line:
#         continue

#     split = line.split()

#     match split[0]:
#         case "var":
#             value = split[2]

#             # if value.isdigit():
#             #     value = int(value)

#             variables[split[1]] = value
#         case "print":
#             print(evaluate(split[1]))
