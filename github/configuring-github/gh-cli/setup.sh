#!/usr/bin/env bash
# Multi-profile GitHub CLI (gh) setup
# gh supports multiple accounts natively since gh 2.40+

set -euo pipefail

# --- 1. Generate SSH keys per profile ---

generate_key() {
    local label="$1"
    local email="$2"
    local keyfile="$HOME/.ssh/id_ed25519_${label}"

    if [[ -f "$keyfile" ]]; then
        echo "Key already exists: $keyfile"
    else
        ssh-keygen -t ed25519 -C "$email" -f "$keyfile" -N ""
        echo "Created: $keyfile"
    fi
}

generate_key "personal"  "abhishek@personal.com"
generate_key "company_a" "abhishek@company-a.com"
generate_key "company_b" "abhishek@company-b.com"

# --- 2. Add keys to ssh-agent ---

eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519_personal
ssh-add ~/.ssh/id_ed25519_company_a
ssh-add ~/.ssh/id_ed25519_company_b

# --- 3. Authenticate gh CLI per account ---

echo ""
echo "=== Authenticating personal account ==="
gh auth login --hostname github.com --git-protocol ssh --web

echo ""
echo "=== To add additional accounts (gh 2.40+): ==="
echo "gh auth login --hostname github.com --git-protocol ssh --web --user your-company-a-username"
echo "gh auth login --hostname github.com --git-protocol ssh --web --user your-company-b-username"

# --- 4. Upload SSH keys to each account ---

echo ""
echo "=== Uploading SSH keys to GitHub ==="
echo "Switch active account first, then upload:"
echo ""
echo "# For personal:"
echo "gh auth switch --user personal-username"
echo "gh ssh-key add ~/.ssh/id_ed25519_personal.pub --title 'Personal Machine'"
echo ""
echo "# For company-a:"
echo "gh auth switch --user company-a-username"
echo "gh ssh-key add ~/.ssh/id_ed25519_company_a.pub --title 'Work Machine - Company A'"
echo ""
echo "# For company-b:"
echo "gh auth switch --user company-b-username"
echo "gh ssh-key add ~/.ssh/id_ed25519_company_b.pub --title 'Work Machine - Company B'"
