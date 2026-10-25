# Removes the trailing comma left by inserting the last `apps` entry before `}`.
import glob
import io
import json
import re

PATTERN = re.compile(r'^(\s*)"cursor": ("[^"]*"),\n(\s*)\},\n', re.M)

for path in sorted(glob.glob(r"src\i18n\locales\*.json")):
    text = io.open(path, encoding="utf-8").read()
    match = PATTERN.search(text)
    if match:
        replacement = '%s"cursor": %s\n%s},\n' % (
            match.group(1),
            match.group(2),
            match.group(3),
        )
        text = text[: match.start()] + replacement + text[match.end() :]
        io.open(path, "w", encoding="utf-8", newline="").write(text)
        print("fixed trailing comma:", path)
    else:
        print("no fix needed:", path)

for path in sorted(glob.glob(r"src\i18n\locales\*.json")):
    data = json.load(io.open(path, encoding="utf-8"))
    print("valid json:", path, "-> apps.cursor =", data["apps"].get("cursor"))
