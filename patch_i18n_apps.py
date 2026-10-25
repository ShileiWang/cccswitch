# Adds the `apps.cursor` brand name to every locale. Brand names stay untranslated.
import io
import re

FILES = [
    r"src\i18n\locales\en.json",
    r"src\i18n\locales\zh.json",
    r"src\i18n\locales\zh-TW.json",
    r"src\i18n\locales\ja.json",
    r"src\i18n\locales\tr.json",
]

for path in FILES:
    text = io.open(path, encoding="utf-8").read()
    if re.search(r'^\s*"cursor"\s*:', text, re.M):
        print("skip (already has apps.cursor):", path)
        continue
    m = re.search(r'^(\s*)"omp"\s*:\s*"Oh My Pi",?\s*\n', text, re.M)
    if not m:
        print("anchor not found:", path)
        continue
    indent = m.group(1)
    line = '%s"omp": "Oh My Pi",\n%s"cursor": "Cursor",\n' % (indent, indent)
    # keep the original line verbatim (it may be the last entry without a comma)
    text = text[: m.start()] + line + text[m.end() :]
    io.open(path, "w", encoding="utf-8", newline="").write(text)
    print("patched:", path)
