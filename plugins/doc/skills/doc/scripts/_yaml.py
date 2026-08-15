#!/usr/bin/env python3
"""Minimal stdlib-only reader for the doc skill's defaults.yaml.

PyYAML is not guaranteed to exist on a machine's default python3, and when it
is missing every cfg() call fails silently and the scripts fall back to their
hardcoded defaults -- meaning defaults.yaml is ignored with no error. This
parser covers the subset the config actually uses: scalars, nested maps,
`key: {}`, and `- item` lists.

Usage:
  _yaml.py <config> get  <dotted.key>   scalar -> value; list/map -> one per line
  _yaml.py <config> tree <dotted.key>   leaf paths of a map, slash-separated
Exits 1 if the key is absent.
"""
import sys


def strip_comment(s):
    out, quote, i = [], None, 0
    while i < len(s):
        c = s[i]
        if quote:
            out.append(c)
            if c == quote:
                quote = None
        elif c in "\"'":
            quote = c
            out.append(c)
        elif c == "#" and (i == 0 or s[i - 1] in " \t"):
            break
        else:
            out.append(c)
        i += 1
    return "".join(out).rstrip()


def unquote(v):
    v = v.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        return v[1:-1]
    return v


def parse(text):
    root = {}
    # each frame: [indent, container, parent_container, key_in_parent]
    stack = [[-1, root, None, None]]
    for raw in text.splitlines():
        line = strip_comment(raw)
        if not line.strip():
            continue
        indent = len(line) - len(line.lstrip())
        body = line.strip()

        while len(stack) > 1 and indent <= stack[-1][0]:
            stack.pop()
        frame = stack[-1]
        cont = frame[1]

        if body.startswith("- "):
            # first list item under a key: retype the placeholder map as a list
            if isinstance(cont, dict):
                lst = []
                frame[1] = lst
                if frame[2] is not None:
                    frame[2][frame[3]] = lst
                cont = lst
            cont.append(unquote(body[2:]))
            continue

        if ":" not in body:
            continue
        key, _, val = body.partition(":")
        key, val = key.strip(), val.strip()
        if val in ("", "{}"):
            child = {}
            cont[key] = child
            stack.append([indent, child, cont, key])
        else:
            cont[key] = unquote(val)
    return root


def lookup(cfg, dotted):
    node = cfg
    for part in dotted.split("."):
        if not isinstance(node, dict):
            return None
        node = node.get(part)
        if node is None:
            return None
    return node


def walk(node, prefix=""):
    if not isinstance(node, dict) or not node:
        if prefix:
            print(prefix)
        return
    for key, sub in node.items():
        path = f"{prefix}/{key}" if prefix else key
        walk(sub, path)


def main():
    if len(sys.argv) != 4:
        sys.exit(2)
    path, mode, dotted = sys.argv[1], sys.argv[2], sys.argv[3]
    with open(path) as fh:
        cfg = parse(fh.read())

    node = lookup(cfg, dotted)

    if mode == "tree":
        walk(node if isinstance(node, dict) else {})
        return

    if node is None:
        sys.exit(1)
    if isinstance(node, list):
        print("\n".join(str(i) for i in node))
    elif isinstance(node, dict):
        if not node:
            sys.exit(1)
        print("\n".join(node.keys()))
    else:
        print(node)


if __name__ == "__main__":
    main()
