@echo off
setlocal enabledelayedexpansion

REM Check for admin privileges
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo This script requires administrator privileges.
    echo Please run as administrator.
    pause
    exit /b 1
)

REM Check if Rust is installed
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust is not installed. Installing Rust...
    powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe"
    rustup-init.exe -y
    del rustup-init.exe
    set PATH=%PATH%;%USERPROFILE%\.cargo\bin
) else (
    echo Rust is already installed.
)

REM Check if Nushell is installed
where nu >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Nushell...
    cargo install nu
) else (
    echo Nushell is already installed.
)

echo Nushell installation complete.
pause