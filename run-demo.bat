@echo off
title Reflex Engine SDK Demo

REM Check if release binary exists
if not exist "target\release\demo.exe" (
    echo Release binary not found. Building first...
    cargo build --release --bin demo
    if errorlevel 1 (
        echo Build failed. Please check your Rust installation.
        pause
        exit /b 1
    )
)

REM Run the demo
target\release\demo.exe
