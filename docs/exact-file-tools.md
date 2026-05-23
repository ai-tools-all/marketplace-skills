# Exact File Tools

Two Pi extension tools were added for safer work on large files:

- `read_exact_lines`: reads a precise 1-indexed line range without injecting synthetic truncation text into the file content.
- `replace_exact_lines`: replaces a precise 1-indexed line range and can reject stale edits with `expectedText`.

Use these when normal file reads are truncated, when a file is too large for reliable context-based patching, or when exact line ranges are safer than broad `oldText` replacement. The write tool uses Pi's file mutation queue and should include `expectedText` for important edits so changed line numbers fail closed instead of corrupting the file.
