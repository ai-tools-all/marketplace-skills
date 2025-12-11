---
description: Scans the team registry to surface warnings, proven configurations, and shortcuts for the current task. Use before starting experiments, refactoring, or complex implementations.
name: advise
---

# Team Advisory

## Execution Checklist
1.  **Pin Goal:** Read the prompt, active file, and recent chat to define the immediate technical objective.
2.  **Scan Registry:** Search `./skills/` (or configured registry) for semantic matches.
3.  **Filter & Prioritize:**
    *   **Priority 1 (Traps):** Files containing "Anti-Patterns," "Gotchas," or "Pitfalls."
    *   **Priority 2 (Shortcuts):** Files containing "Magic Numbers," config blocks, or proven commands.
    *   **Priority 3 (Context):** Files with matching dependencies or stack versions.
4.  **Synthesize:** Do not dump file contents. Extract specific values and warnings.

## Output Template
If relevant skills are found, use this exact format:

```markdown
### 🛑 Known Pitfalls
- <Specific "Do not do X" warning tied to current stack>
- <Root cause of previous failures (e.g., OOM on specific batch size)>

### ⚡ Accelerated Path
- <Copyable config / command / hyperparameter value>
- <Snippet of "Happy Path" code>

### 🧠 Source Context
- **Based on:** <Skill File Name> (<Date>)
- **Relevance:** <Why it applies (e.g., "Same dataset size", "v2 API")>
- **Caveats:** <Version drift or context mismatch warnings>