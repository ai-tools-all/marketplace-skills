#!/usr/bin/env bash
# Quick profile switching helpers
# Source this in your .bashrc/.zshrc

# Switch gh CLI active account + print confirmation
ghswitch() {
    local user="$1"
    if [[ -z "$user" ]]; then
        echo "Usage: ghswitch <github-username>"
        echo ""
        echo "Available accounts:"
        gh auth status
        return 1
    fi
    gh auth switch --user "$user"
    echo "Active account: $(gh api user --jq .login)"
}

# List all authenticated accounts
ghwho() {
    gh auth status
}

# Clone using correct host alias based on org
# Usage: ghclone company-a/repo-name
ghclone() {
    local repo="$1"
    local org="${repo%%/*}"

    case "$org" in
        company-a)
            git clone "git@github-company-a:${repo}.git"
            ;;
        company-b)
            git clone "git@github-company-b:${repo}.git"
            ;;
        *)
            git clone "git@github.com:${repo}.git"
            ;;
    esac
}

# Create a repo under the correct account
# Usage: ghcreate my-new-repo --private
ghcreate() {
    local name="$1"
    shift
    gh repo create "$name" "$@"
}
