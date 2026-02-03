@echo off
setlocal enabledelayedexpansion

echo ========================================
echo  Featured Offsets Finder - C++ Build
echo ========================================
echo.

REM Check if CMake is installed
where cmake >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] CMake not found!
    echo.
    echo Please install CMake from: https://cmake.org/download/
    echo.
    pause
    exit /b 1
)

REM Check if Visual Studio is available
where cl >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [INFO] Visual Studio compiler not found in PATH
    echo [INFO] Attempting to locate Visual Studio...
    
    REM Try to find Visual Studio 2022
    if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" (
        call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
    ) else if exist "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat" (
        call "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat"
    ) else if exist "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat" (
        call "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat"
    ) else if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat" (
        call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
    ) else (
        echo [ERROR] Visual Studio not found!
        echo.
        echo Please install Visual Studio or run this script from a Developer Command Prompt
        echo Download: https://visualstudio.microsoft.com/downloads/
        echo.
        pause
        exit /b 1
    )
)

echo [1/4] Cleaning previous build...
if exist build (
    rmdir /s /q build
)

echo [2/4] Creating build directory...
mkdir build
cd build

echo [3/4] Configuring with CMake...
cmake .. -G "Visual Studio 17 2022" -A x64
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] CMake configuration failed!
    cd ..
    pause
    exit /b 1
)

echo [4/4] Building Release version...
cmake --build . --config Release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed!
    cd ..
    pause
    exit /b 1
)

cd ..

echo.
echo ========================================
echo   BUILD SUCCESSFUL!
echo ========================================
echo.
echo Executable location:
echo   build\bin\Release\offsets-finder.exe
echo.
echo To run the program:
echo   cd build\bin\Release
echo   offsets-finder.exe
echo.

pause
