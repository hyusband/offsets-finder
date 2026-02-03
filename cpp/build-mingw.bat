@echo off
echo ========================================
echo   Quick Build (MinGW/GCC)
echo ========================================
echo.

REM Check if g++ is available
where g++ >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] g++ not found!
    echo.
    echo Please install MinGW-w64 or use build.bat for Visual Studio
    echo Download: https://www.mingw-w64.org/
    echo.
    pause
    exit /b 1
)

echo [INFO] Compiling with g++...

g++ -std=c++17 -O3 -Wall -Wextra ^
    -Iinclude ^
    src/main.cpp ^
    src/models.cpp ^
    src/config.cpp ^
    src/scanner.cpp ^
    src/exporter.cpp ^
    src/ui.cpp ^
    -o offsets-finder.exe

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Compilation failed!
    pause
    exit /b 1
)

echo.
echo ========================================
echo   BUILD SUCCESSFUL!
echo ========================================
echo.
echo Executable: offsets-finder.exe
echo.
echo To run: offsets-finder.exe
echo.

pause
