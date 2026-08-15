---
name: ast-grep
description: Use when writing, reviewing, debugging, or applying ast-grep rules/codemods; when asked for structural search, AST-based code matching, YAML rule configuration, pattern/kind/regex matching, relational/composite rules, or reusable ast-grep utils.
---

# ast-grep Rule Authoring

Use this skill whenever a task can be solved with ast-grep structural search or rewrite rules. Prefer ast-grep over plain regex when code shape, AST node kind, parent/child relationship, or language-aware matching matters.

Source reference downloaded from ast-grep docs: `rule.md`.

## Fast Workflow

1. Identify the language and the target AST node shape.
2. Start with the smallest **atomic rule** (`pattern`, `kind`, `regex`, `nthChild`, or `range`).
3. Add **relational rules** (`inside`, `has`, `precedes`, `follows`) only when structure around the target matters.
4. Combine with **composite rules** (`all`, `any`, `not`) when the target node has multiple constraints.
5. Extract repeated checks into `utils` and refer to them with `matches`.
6. Test the rule against real files before using it for edits or rewrites.

## Rule Object Cheat Sheet

### Atomic Rules: match the current node

Use these to match individual AST nodes by their own properties.

```yaml
pattern: console.log($ARG)
```
Matches code structure. Meta-variables like `$ARG` capture one node; `$$` is useful for variadic/looser captures in patterns.

```yaml
pattern:
  context: '{ key: value }'
  selector: pair
```
Use `context` plus `selector` when a fragment is ambiguous without surrounding syntax.

```yaml
kind: if_statement
```
Matches by AST node kind.

```yaml
regex: ^regex.+$
```
Matches the node text with a Rust regular expression.

```yaml
nthChild: 1
```
Matches the 1-based index among named siblings.

```yaml
nthChild:
  position: 2
  reverse: true
  ofRule: { kind: argument_list }
```
Advanced sibling position: `position`, `reverse` to count from the end, and `ofRule` to filter siblings.

```yaml
range:
  start: { line: 0, column: 0 }
  end: { line: 0, column: 13 }
```
Matches by source span. Coordinates are 0-based; start is inclusive, end is exclusive.

### Relational Rules: match AST relationships

Use these when the target node depends on surrounding structure.

```yaml
inside:
  kind: function_declaration
```
Target must appear inside a parent or ancestor matching the sub-rule.

```yaml
has:
  kind: method_definition
```
Target must have a child or descendant matching the sub-rule.

```yaml
has:
  kind: statement_block
  field: body
```
`field` restricts `has`/`inside` to semantic roles such as a function body.

```yaml
precedes:
  pattern: function $FUNC() { $$ }
```
Target must appear before another node matching the sub-rule.

```yaml
follows:
  pattern: let x = 10;
```
Target must appear after another node matching the sub-rule.

```yaml
inside:
  kind: function_declaration
  stopBy: end
```
`stopBy` controls how far relational search proceeds instead of only immediate neighbors.

### Composite Rules: boolean logic on one target node

Composite operations all apply to the same current target node.

```yaml
all:
  - pattern: const $VAR = $VALUE
  - has: { kind: string_literal }
```
Node must satisfy every rule.

```yaml
any:
  - pattern: let $X = $Y
  - pattern: const $X = $Y
```
Node must satisfy at least one rule.

```yaml
not:
  pattern: console.log($$)
```
Node must not satisfy the sub-rule.

```yaml
matches: is-function-call
```
Delegates to a reusable utility rule.

### Utility Rules: reuse and modularity

Local utils live in the same config file:

```yaml
rules:
  - id: find-my-pattern
    rule:
      matches: my-local-check
utils:
  my-local-check:
    kind: identifier
    regex: '^my'
```

Global/project utils live in separate YAML files under configured `utilsDirs`:

```yaml
# utils/my-global-check.yml
id: my-global-check
language: javascript
rule:
  kind: variable_declarator
  has:
    kind: number_literal
```

## Authoring Heuristics

- Prefer `pattern` for recognizable code snippets.
- Prefer `kind` when matching syntax categories independent of exact text.
- Prefer `regex` only for text inside an already AST-scoped node; avoid replacing AST structure with pure regex.
- Use `context`/`selector` for fragments that cannot parse alone, such as object pairs, arguments, or class members.
- Use `inside` and `has` to express parent/child intent instead of broad patterns that accidentally overmatch.
- Use `field` when a child role matters; it is more precise than matching any descendant.
- Use `all` to tighten a rule; use `any` to support equivalent syntactic forms; use `not` to exclude false positives.
- Keep utility rules small and named by intent, not by implementation detail.

## Review Checklist

Before trusting an ast-grep rule:

- Does the rule target the node you intend to report or rewrite, not merely a convenient ancestor?
- Are metavariables specific enough to avoid accidental captures?
- Would comments, nested expressions, optional syntax, or formatting changes still match correctly?
- If using `regex`, is it scoped by AST first?
- If using relational rules, is the relationship direction correct (`inside` vs `has`, `precedes` vs `follows`)?
- If using `all`/`any`/`not`, are all clauses constraints on the same target node?
- Are common false positives excluded with `not`, `field`, or a stricter `kind`?

## Minimal Rule Skeleton

```yaml
id: descriptive-rule-id
language: javascript
rule:
  all:
    - pattern: console.log($ARG)
    - inside:
        kind: function_declaration
message: Avoid console.log inside functions.
severity: warning
```
