# Adds the Cursor icon to the extracted icon index / metadata.
import io

IDX = r"src\icons\extracted\index.ts"
META = r"src\icons\extracted\metadata.ts"

CURSOR_SVG = (
    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1em\" height=\"1em\" "
    "style=\"flex:none;line-height:1\" viewBox=\"0 0 64 64\"><title>Cursor</title>"
    "<g fill=\"currentColor\"><path d=\"M10.2 8.5h26.4c3 0 4.6 2 4.6 4.4v18.8c0 2.3-1.7 "
    "4-4 4H21.7l-4.3 4.1v-4.1H10.2c-2.3 0-4-1.7-4-4V12.9c0-2.4 1.6-4.4 4-4.4z\"/>"
    "<path d=\"M30.8 34.3h9.4c4.1 0 7.4 3.2 7.4 7.2v12.7c0 2.7-2.2 4.9-4.9 4.9h-4l-4.6 "
    "4.4v-4.4h-3.3c-2.7 0-4.9-2.2-4.9-4.9V41.5c0-4 3.3-7.2 7.3-7.2z\" opacity=\".38\"/>"
    "</g></svg>"
)


def insert_after(text, anchor, addition):
    i = text.index(anchor)
    # move to the end of that entry line
    j = text.index("\n", i) + 1
    return text[:j] + addition + text[j:]


idx = io.open(IDX, encoding="utf-8").read()
if "\n  cursor:" not in idx:
    idx = insert_after(idx, "  omp: `", "  cursor: `%s`,\n" % CURSOR_SVG)
    io.open(IDX, "w", encoding="utf-8", newline="").write(idx)
    print("index patched")
else:
    print("index already has cursor")

meta = io.open(META, encoding="utf-8").read()
if "\n  cursor: {" not in meta:
    entry = (
        "  cursor: {\n"
        "    name: \"cursor\",\n"
        "    displayName: \"Cursor\",\n"
        "    category: \"ai-provider\",\n"
        "    shape: \"tile\",\n"
        "    keywords: [\"cursor\", \"cursor ide\", \"anysphere\"],\n"
        "    defaultColor: \"currentColor\",\n"
        "  },\n"
    )
    meta = insert_after(meta, "  openclaw: {", entry)
    io.open(META, "w", encoding="utf-8", newline="").write(meta)
    print("metadata patched")
else:
    print("metadata already has cursor")
