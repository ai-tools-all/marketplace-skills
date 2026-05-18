# Multi-Profile GitHub Configuration

Managing multiple GitHub identities (personal, work-a, work-b) from a single machine using **gitconfig**, **SSH config**, and **gh CLI**.

## How It Works

```
┌─────────────────────────────────────────────────────────┐
│  ~/.gitconfig (global)                                  │
│    ├── includeIf ~/work/company-a/ → .gitconfig-company-a│
│    ├── includeIf ~/work/company-b/ → .gitconfig-company-b│
│    └── includeIf ~/personal/       → .gitconfig-personal │
├─────────────────────────────────────────────────────────┤
│  ~/.ssh/config                                          │
│    ├── Host github-company-a → id_ed25519_company_a     │
│    ├── Host github-company-b → id_ed25519_company_b     │
│    └── Host github.com       → id_ed25519_personal      │
├─────────────────────────────────────────────────────────┤
│  gh CLI (2.40+)                                         │
│    ├── gh auth switch --user personal-username          │
│    ├── gh auth switch --user company-a-username         │
│    └── gh auth switch --user company-b-username         │
└─────────────────────────────────────────────────────────┘
```

## Setup Steps

1. **Generate SSH keys** — one per identity (`gh-cli/setup.sh`)
2. **Configure SSH** — host aliases route to correct key (`ssh-config/config`)
3. **Configure git** — conditional includes set name/email per directory (`gitconfig/`)
4. **Authenticate gh** — multi-account login and switching (`gh-cli/`)
5. **Upload keys** — push public keys to each GitHub account via `gh ssh-key add`

## Directory Layout

```
configuring-github/
├── README.md
├── gitconfig/
│   ├── .gitconfig              # Global config with conditional includes
│   ├── .gitconfig-company-a    # Company A overrides
│   ├── .gitconfig-company-b    # Company B overrides
│   └── .gitconfig-personal     # Personal overrides
├── ssh-config/
│   └── config                  # SSH host aliases per identity
└── gh-cli/
    ├── setup.sh                # One-time setup script
    └── switching-profiles.sh   # Shell helpers for daily use
```

## Daily Workflow

```bash
# Check who you're authenticated as
gh auth status

# Switch active gh account
gh auth switch --user company-a-username

# Git automatically picks up identity based on directory
cd ~/work/company-a/some-repo
git commit -m "uses company-a email automatically"

# Clone via correct SSH alias (handled by url.insteadOf in gitconfig)
git clone git@github.com:company-a/repo.git
# ↑ automatically rewritten to git@github-company-a:company-a/repo.git
```

## Requirements

- Git 2.36+ (for `includeIf` and SSH signing)
- GitHub CLI 2.40+ (for multi-account support)
- OpenSSH with Ed25519 support
