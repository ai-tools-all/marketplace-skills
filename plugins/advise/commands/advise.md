---
description: Consult the team registry to surface warnings, proven configurations, and shortcuts relevant to the current task.
---

# Team Advisory Command

## Phase 1: Context & Retrieval
1. **Analyze Current Intent:** Read the user's current prompt, active file selection, and recent chat history to determine the immediate technical goal (e.g., "Starting a pruning experiment," "Refactoring auth middleware").
2. **Search Registry:** Scan the `./skills/` (or designated team registry) folder. Look for semantic matches, not just keyword matches (e.g., if user is doing "cleanup," look for "refactoring" or "pruning").
3. **Identify Relevancy:** Prioritize Skill files that contain:
   - **Anti-Patterns/Gotchas** related to the current stack.
   - **Configuration/Hyperparameters** that were previously verified.
   - **Dependencies** that match the current environment.

## Phase 2: Synthesis (The "Senior Engineer" Filter)
Do not dump the content of the files. Synthesize specific advice for the *current* context:
1. **Extract Warnings First:** If a relevant Skill file lists a "Trap" or "Anti-Pattern," surface this immediately.
2. **Extract "Magic Numbers":** If the previous attempt found specific values (timeout durations, learning rates, buffer sizes) that worked, present them as the starting point.
3. **Compare Contexts:** If the previous skill was for a different version or slightly different use case, explicitly state the assumption (e.g., "This advice is from v2, verify validity for v3").

## Phase 3: Advisory Output
Format the response to stop the user from wasting time. Use the following structure:

### 🛑 STOP & READ: Known Pitfalls
*   *List specific "Don'ts" found in the registry.*
*   *(e.g., "Do not use the default HNSW index for this dataset size; previous experiments showed it caused OOM errors.")*

### ⚡ Accelerated Path
*   *Provide the "Cheat Sheet" from previous learnings.*
*   *Copy-pasteable config blocks or command arguments that are known to work.*

### 🧠 Source Context
*   "Based on [Skill File Name] created on [Date]."
*   *Brief summary of why that previous session is relevant to this one.*

## Phase 4: Negative Result
If no relevant skills/retrospectives are found in the registry:
*   State clearly: "No matching prior art found in the team registry."
*   Encourage the user: "Proceed with caution. Please run `/retrospective` after this session to create the first Skill file for this topic."
