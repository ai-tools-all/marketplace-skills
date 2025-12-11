---
description: Analyzes the current session to synthesize a compliant Skill file with learnings and anti-patterns. Use after a successful task completion or a solved debugging session.
name: retrospective
---

# Retrospective & Skill Generation

## Analysis Checklist
1.  **Identify Objective:** What was the user trying to achieve?
2.  **Trace Critical Path:** What was the exact sequence of *working* steps?
3.  **Isolate Anti-Patterns:** What failed? Why? (Syntax, logic, version mismatch?)
4.  **Verify Compliance:** Ensure the output will adhere to "Skill Authoring Best Practices."

## Output Template (The Skill File)
Generate a new markdown file. The content MUST follow this structure:

```markdown
---
description: <Third-person summary of action and trigger context. Max 1024 chars.>
name: <gerund-form-name> (e.g., managing-db, not db-manager)
---

# <Title>

## Prerequisites
- <Required libraries, API keys, or environment states>

## The Happy Path
<The final, corrected code or command sequence only.>
<If code > 50 lines, instruct user to create a script file.>

## 🛑 Anti-Patterns & Gotchas
- **Do not:** <Specific action that failed>
- **Because:** <Root cause>
- **Instead:** <The fix>

## Verification
<One command to verify success>
```

## Quality Control Rules
- **Name:** Must be lowercase, numbers, hyphens only. No reserved words.
- **Description:** Must NOT start with "I can help..." or "Use this to...". Start with the verb (e.g., "Deploys...", "Analyzes...").
- **Code:** Do not include broken attempts in the "Happy Path." Move them to "Anti-Patterns."
- **Inconclusive Sessions:** If the session did not result in a solution, do not generate a file. Report: "Session inconclusive; lacks a verifiable solution."
```