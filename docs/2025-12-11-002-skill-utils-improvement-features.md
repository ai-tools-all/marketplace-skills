Each skill follows the same layout:

plugins/training/experiment-name/
├── .claude-plugin/
│   └── plugin.json          # Metadata and trigger conditions
├── skills/experiment-name/
│   └── SKILL.md             # The main knowledge document
├── references/
│   ├── experiment-log.md    # Daily experiment notes
│   └── troubleshooting.md   # Error → solution mappings
└── scripts/
    └── (reusable code)




----

# Research Skills Registry

## Commands

### /advise
Search the skills registry for relevant experiments before starting new work.
1. Read the user's goal
2. Search plugins/ for related skills by scanning description fields
3. Summarize relevant findings: what worked, what failed, recommended parameters

### /retrospective  
Save learnings from the current session as a new skill.
1. Summarize key findings from the conversation
2. Create a new plugin folder using templates/experiment-skill-template/
3. Fill in SKILL.md with: goal, what worked, what failed, final parameters
4. Create a branch and open a PR to main

## Skill Template
Use templates/experiment-skill-template/ as the base for new skills.

## Rules
- Every skill needs a specific description field with trigger conditions
- Always include a "Failed Attempts" table
- Include exact hyperparameters, not vague advice
