@echo off
echo ============================================================
echo           SPIRAPI WIKI UPLOAD SCRIPT
echo ============================================================
echo.

echo [INFO] Starting SpiraPi Wiki Upload Process...
echo.

REM Check if wiki directory exists (from scripts directory)
if not exist "..\wiki" (
    echo [ERROR] Wiki directory not found!
    echo Please ensure the wiki directory exists in the project root.
    pause
    exit /b 1
)

REM Navigate to wiki directory (from scripts directory)
cd ..\wiki
echo [INFO] Working in directory: %CD%
echo.

REM Check git status
git status >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Not a git repository or git not available!
    pause
    exit /b 1
)

REM Check remote URL
for /f "tokens=*" %%i in ('git remote get-url origin') do set REMOTE_URL=%%i
echo [INFO] Confirmed: Working in %REMOTE_URL%
echo.

REM Check current status
echo [INFO] Current Git Status:
git status --porcelain
echo.

REM Add all files
echo [INFO] Adding files to git...
git add .
echo [INFO] Files added successfully
echo.

REM Check what will be committed
echo [INFO] Files staged for commit:
git diff --cached --name-only
echo.

REM Commit changes (only if there are changes)
git diff --cached --quiet
if errorlevel 1 (
    echo [INFO] Committing changes...
    git commit -m "docs: add comprehensive SpiraPi wiki documentation"
    echo [INFO] Changes committed successfully!
) else (
    echo [INFO] No changes to commit
)
echo.

REM Push to GitHub (use master branch for wiki)
echo [INFO] Pushing to GitHub...
git push origin master
if errorlevel 1 (
    echo [ERROR] Failed to push to GitHub!
    echo [INFO] This might be normal if no changes were made
) else (
    echo [INFO] Successfully pushed to GitHub!
)
echo.

REM Show final status
echo [INFO] Final Status:
git status
echo.

echo [SUCCESS] Wiki upload process completed!
echo [INFO] Your wiki is available at: https://github.com/iyotee/SpiraPi/wiki
echo.

pause
