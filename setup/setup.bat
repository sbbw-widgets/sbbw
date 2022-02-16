@echo off

goto check_Permissions

:check_Permissions
    echo Administrative permissions required. Detecting permissions...

    net session >nul 2>&1
    if not %errorLevel% == 0 (
        echo [ X ] Run as administrator.
        pause >nul
        exit /b 1
    )


SET PATH_TO_INSTALL=C:\Program Files\sbbw
SET CURRENT_DIR=%~dp0

echo Installing sbbw
timeout /t 2 > NUL

if exist "%PATH_TO_INSTALL%\sbbw.exe" (
    echo sbbw already installed
    pause >nul
    exit 1
)
if not exist "%CURRENT_DIR%\sbbw.exe" (
    echo sbbw binary not found
    pause >nul
    exit 1
)
if not exist "%CURRENT_DIR%\sbbw-widget.exe" (
    echo sbbw-widget binary not found
    pause >nul
    exit 1
)

:: Add sbbw to PATH
if not exist "%PATH_TO_INSTALL%" (
    mkdir "%PATH_TO_INSTALL%"
)

setx /M PATH "%PATH%;%PATH_TO_INSTALL%"

copy "%CURRENT_DIR%\sbbw.exe" "%PATH_TO_INSTALL%"
copy "%CURRENT_DIR%\sbbw-widget.exe" "%PATH_TO_INSTALL%"

echo [ OK ] Sbbw Installed
timeout /t 2 > NUL

echo [ OK ] Done, you can close this

pause >nul
