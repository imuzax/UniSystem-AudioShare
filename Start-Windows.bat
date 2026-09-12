@echo off
color 0A
title UniSystem AudioShare - Startup

echo ====================================================
echo        UniSystem AudioShare Windows Startup        
echo ====================================================
echo.
echo Please ensure you have Node.js and Rust installed!
echo.
echo [1/3] Preparing to launch the Desktop GUI...
cd unisystem-tauri

echo.
echo [2/3] Installing NPM dependencies (this may take a moment on first run)...
call npm install

echo.
echo [3/3] Compiling and starting Tauri...
echo The GUI window will open shortly.
echo.
call npm run tauri dev

pause
