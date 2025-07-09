import argparse, json

ap = argparse.ArgumentParser()
ap.add_argument("--save")
ap.add_argument("squares", nargs="?", default="squares.json")
args = ap.parse_args()

with open("usa_5.txt") as f:
    dict = f.read().splitlines()
dict_set = set(dict)

with open(args.squares) as f:
    data = json.load(f)

def transpose(s):
    result = ["" for _ in range(5)]
    for i in range(5):
        for j in range(5):
            result[j] += s[i][j]
    return result

def words(s):
    return set(s) | set(transpose(s))

def check_square(s):
    for w in words(s):
        if w not in dict_set:
            return False
    return True

print(f"squares: {len(data)}")

bad = [s for s in data if not check_square(s)]
print(f"bad: {len(bad)}")

unique = set(tuple(s) for s in data)
print(f"unique: {len(unique)}")

doubly = set(s for s in unique if len(words(s)) == 10)
print(f"doubly: {len(doubly)}")

canonical = set(s for s in doubly if transpose(s)[0] > s[0])
print(f"canonical: {len(canonical)}")

if args.save:
    with open(args.save, "w") as f:
        json.dump(sorted(list(canonical)), f)
