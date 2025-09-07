@echo off
setlocal

REM Check for admin rights
openfiles >nul 2>&1
if errorlevel 1 (
    echo This script requires administrator privileges.
    echo Please right-click and run as administrator, or approve the UAC prompt.
    pause
    exit /b 1
)


REM Check if pyinstaller is installed
pyinstaller --version >nul 2>&1
if errorlevel 1 (
    echo PyInstaller is not installed. Please install it with:
    echo   pip install pyinstaller
    pause
    exit /b 1
)


REM Always build the project using pyinstaller (no prompt)
if exist main.py (
    pyinstaller --onefile main.py
) else (
    echo main.py not found!
    exit /b 1
)

REM Add dist folder to PATH for current session
set /p ADDPATH="Do you want to add the dist folder to your PATH for this session? (y/n): "
if /i "%ADDPATH%"=="y" (
    set "PATH=%CD%\dist;%PATH%"
    echo dist folder added to PATH for this session.
) else (
    echo Skipping PATH update for this session.
)

REM Optionally add dist to user environment PATH permanently
set /p PERMPATH="Do you want to add the dist folder to your user PATH permanently? (y/n): "
if /i "%PERMPATH%"=="y" (
    setx PATH "%CD%\dist;%PATH%"
    echo dist folder added to user PATH permanently.
) else (
    echo Skipping permanent PATH update.
)

REM For PowerShell, suggest how to update $env:Path
if "%ComSpec%"=="%SystemRoot%\system32\cmd.exe" (
    echo.
    echo To add dist to your PowerShell session, run:
    echo   $env:Path = "$PWD\dist;" + $env:Path
)

endlocal
pause
