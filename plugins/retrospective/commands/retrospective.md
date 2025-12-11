---
description: Analyze session context to synthesize a robust Skill file, preserving critical learnings and anti-patterns, and prepare a PR.
name: retrospective
---

# Retrospective & Skill Generation

## Phase 1: Deep Analysis
Read the entire active session. Do not just summarize; **synthesize** the workflow by answering:
1.  **The Objective:** What was the user ultimately trying to achieve?
2.  ** The Critical Path:** What was the exact sequence of steps that led to the final solution?
3.  **The Traps (Root Cause Analysis):** Look at every error or failed attempt. Why did it fail?
    *   *Was it a syntax hallucination?*
    *   *Was it a logical flow error?*
    *   *Was it a missing dependency?*
    *   *Note: These mistakes are high-value context. They define the "Anti-Patterns" section.*

## Phase 2: Skill File Composition
Create a new Skill file (markdown format) that serves as a **standalone guide** for a future developer (or AI) to repeat this task without errors.

**Required Structure:**
1.  **Metadata:** Title, Intent, and Description.
2.  **Prerequisites:** What specific libraries, API keys, or environment states are required?
3.  **The Happy Path (Numbered Steps):**
    *   Provide the *final, corrected* code snippets only.
    *   Ensure code is modular and comments explain *why* specific parameters are chosen.
4.  **Anti-Patterns & Gotchas (Crucial):**
    *   Explicitly list the mistakes made during this session.
    *   Format as: "Do not do X; it results in error Y. Instead, ensure Z."
5.  **Verification:** A specific command or check to confirm the task succeeded.

## Phase 3: Quality Control & Output
- **Constraint:** If the session was exploratory and yielded no working solution, **STOP**. Report: "Session inconclusive; lacks a verifiable solution for a Skill file."
- **Action:** If successful, write the file to the `./skills/` directory (or equivalent) and generate a PR description summarizing the new capability.