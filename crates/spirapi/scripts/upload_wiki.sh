#!/bin/bash

# ============================================================
#           SPIRAPI WIKI UPLOAD SCRIPT (Linux)
# ============================================================

echo "============================================================"
echo "           SPIRAPI WIKI UPLOAD SCRIPT (Linux)"
echo "============================================================"
echo

echo "[INFO] Starting SpiraPi Wiki Upload Process..."
echo

# Check if wiki directory exists (from scripts directory)
if [ ! -d "../wiki" ]; then
    echo "[ERROR] Wiki directory not found!"
    echo "Please ensure the wiki directory exists in the project root."
    exit 1
fi

# Navigate to wiki directory (from scripts directory)
cd ../wiki
echo "[INFO] Working in directory: $(pwd)"
echo

# Check git status
if ! git status >/dev/null 2>&1; then
    echo "[ERROR] Not a git repository or git not available!"
    exit 1
fi

# Check remote URL
REMOTE_URL=$(git remote get-url origin 2>/dev/null)
if [ -z "$REMOTE_URL" ]; then
    echo "[ERROR] No remote origin found!"
    exit 1
fi
echo "[INFO] Confirmed: Working in $REMOTE_URL"
echo

# Check current status
echo "[INFO] Current Git Status:"
git status --porcelain
echo

# Add all files
echo "[INFO] Adding files to git..."
git add .
echo "[INFO] Files added successfully"
echo

# Check what will be committed
echo "[INFO] Files staged for commit:"
git diff --cached --name-only
echo

# Commit changes (only if there are changes)
if ! git diff --cached --quiet; then
    echo "[INFO] Committing changes..."
    git commit -m "docs: add comprehensive SpiraPi wiki documentation"
    echo "[INFO] Changes committed successfully!"
else
    echo "[INFO] No changes to commit"
fi
echo

# Push to GitHub (use master branch for wiki)
echo "[INFO] Pushing to GitHub..."
if git push origin master; then
    echo "[INFO] Successfully pushed to GitHub!"
else
    echo "[ERROR] Failed to push to GitHub!"
    echo "[INFO] This might be normal if no changes were made"
fi
echo

# Show final status
echo "[INFO] Final Status:"
git status
echo

echo "[SUCCESS] Wiki upload process completed!"
echo "[INFO] Your wiki is available at: https://github.com/iyotee/SpiraPi/wiki"
echo

# Make the script executable
chmod +x upload_wiki.sh
