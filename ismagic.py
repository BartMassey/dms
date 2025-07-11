def transpose(s):
    result = [""] * 5
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

def print_square(s, f):
    for i in range(5):
        print(s[i], file=f)

def is_magic(s):
    t = transpose(s)
    for i in range(5):
        if s[i] != t[i]:
            return False
    return True

print(f"squares: {len(data)}")

bad = [s for s in data if not check_square(s)]
print(f"bad: {len(bad)}")

unique = set(tuple(s) for s in data)
print(f"unique: {len(unique)}")

magic = set(s for s in unique if is_magic(s))
print(f"magic: {len(magic)}")

canonical = set(s for s in unique if transpose(s)[0] > s[0])
print(f"canonical: {len(canonical)}")

doubly = set(s for s in canonical if len(words(s)) == 10)
print(f"doubly: {len(doubly)}")


if args.output:
    squares = sorted(list(canonical))
    with open(args.output, "w") as f:
        if args.save_format == "json":
            json.dump(squares, f)
        elif args.save_format == "txt":
            print_square(squares[0], f)
            for s in squares[1:]:
                print(file=f)
                print_square(s, f)
        else:
            raise Exception(f"{args.output_format}: unknown format")
