import json
import pathlib
import subprocess

data = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--filter-platform", "aarch64-apple-ios", "--format-version", "1"], text=True))
nodes = {item["id"]: item for item in data["resolve"]["nodes"]}
todo = [item["id"] for item in data["packages"] if item["name"] == "range"]
seen = set()
while todo:
    key = todo.pop()
    if key not in seen:
        seen.add(key)
        todo.extend(item["pkg"] for item in nodes[key]["deps"])
text = []
for item in data["packages"]:
    if item["id"] not in seen or item["source"] is None:
        continue
    root = pathlib.Path(item["manifest_path"]).parent
    files = sorted(path for path in root.rglob("*") if path.is_file() and
        (path.name.lower().startswith(("license", "licence", "copying", "copyright")) or path.name.lower() == "ofl.txt"))
    extra = pathlib.Path("engine/licenses") / f"{item['name']}-{item['version']}.LICENSE"
    if not files and extra.is_file():
        text.append(f"\n{item['name']} {item['version']}\n{item.get('repository') or ''}\nLicense: {item['license']}\n{extra.read_text()}\n")
        continue
    if not files:
        raise ValueError(f"Missing license text: {item['name']}")
    text.append(f"\n{item['name']} {item['version']}\n{item.get('repository') or ''}\nLicense: {item['license']}\n")
    for path in files:
        text.append(f"\n{path.relative_to(root)}\n{path.read_text(errors='replace')}\n")
pathlib.Path("build/range-licenses.txt").write_text("".join(text))
print(f"Collected notices for {len(text)} license sections")
