@echo off
setlocal

:: Check if the required argument is provided
if "%1"=="" (
    echo Usage: %~n0 [project_name]
    echo Example: %~n0 my_project
    exit /b 1
)

:: Variables for paths and filenames
set TARGET=wasm32-unknown-unknown
set OUT_DIR=.\target\%TARGET%\release\out
set WASM_FILE=.\target\%TARGET%\release\%1.wasm

:: Step 1: Build the project
echo Building the project...
cargo build --release --target %TARGET%
if errorlevel 1 (
    echo Error: Failed to build the project.
    exit /b 1
)

:: Step 2: Generate bindings with wasm-bindgen
echo Generating bindings with wasm-bindgen...
wasm-bindgen --no-typescript --target web --out-dir %OUT_DIR% --out-name %1 "%WASM_FILE%"
if errorlevel 1 (
    echo Error: Failed to generate bindings with wasm-bindgen.
    exit /b 1
)

:: Step 3: Copy assets
echo Copying assets...
xcopy .\assets %OUT_DIR%\assets /s /i /y
if errorlevel 1 (
    echo Error: Failed to copy assets.
    exit /b 1
)

:: Step 4: Create a basic index.html file
echo Creating index.html...
(
    echo ^<!DOCTYPE html^>
    echo ^<html lang="en"^>
    echo ^<head^>
    echo     ^<meta charset="UTF-8"^>
    echo     ^<meta name="viewport" content="width=device-width, initial-scale=1.0"^>
    echo     ^<title^>%1^</title^>
    echo ^</head^>
    echo ^<body^>
    echo     ^<script type="module"^>
    echo         import init from './%1.js';
    echo         init^(^).catch^(^(error^) ^=^> {
    echo             if ^(!error.message.startsWith^("Using exceptions for control flow, don't mind me. This isn't actually an error!"^)^) {
    echo                 throw error;
    echo             }
    echo         }^);
    echo     ^</script^>
    echo ^</body^>
    echo ^</html^>
) > "%OUT_DIR%\index.html"

if errorlevel 1 (
    echo Error: Failed to create index.html.
    exit /b 1
)

echo Done!
