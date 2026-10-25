import io

TARGETS = [
    ("tests/msw/state.ts", [80, 96, 214, 344]),
]

for path, line_numbers in TARGETS:
    with io.open(path, encoding="utf-8", newline="") as fh:
        lines = fh.read().splitlines(keepends=True)
    for ln in sorted(line_numbers, reverse=True):
        line = lines[ln - 1]
        indent = line[: len(line) - len(line.lstrip())]
        value = "{}" if "{}" in line else '""'
        lines.insert(ln, "%scursor: %s,\n" % (indent, value))
    with io.open(path, "w", encoding="utf-8", newline="") as fh:
        fh.writelines(lines)
    print("patched", path)
