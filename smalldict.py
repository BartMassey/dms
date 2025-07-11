import argparse, json, sys

ap = argparse.ArgumentParser()
ap.add_argument("-o", "--output")
ap.add_argument("-n", "--nsquares", type=int, default=100)
ap.add_argument("squares", nargs="?", default="squares.json")
args = ap.parse_args()

if args.output:
    outfile = open(args.output, "w")
else:
    outfile = sys.stdout

def transpose(s):
    result = ["" for _ in range(5)]
    for i in range(5):
        for j in range(5):
            result[j] += s[i][j]
    return result

def words(s):
    for i in range(5):
        yield s[i]
    t = transpose(s)
    for i in range(5):
        yield t[i]

with open(args.squares, "r") as f:
    data = json.load(f)

ws = list(set(w for i in range(args.nsquares) for w in words(data[i])))

for w in sorted(ws):
    print(w, file=outfile)
