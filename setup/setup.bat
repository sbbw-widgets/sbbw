@echo off

goto check_Permissions

:check_Permissions
    echo Administrative permissions required. Detecting permissions...

    net session >nul 2>&1
    if not %errorLevel% == 0 (
        echo Failure: Run as administrator.
        pause >nul
        exit /b 1
    )


SET PATH_TO_INSTALL=C:\Program Files\sbbw

echo "Installing sbbw"
sleep 2

IF exist "%PATH_TO_INSTALL%\sbbw.exe" ]] (
  echo "sbbw already installed"
  exit 1
)
IF not exist sbbw.exe (
  echo "sbbw binary not found"
  exit 1
)
IF not exist sbbw-widget.exe (
  echo "sbbw-widget binary not found"
  exit 1
)

:: Add sbbw to PATH
IF not exist "%PATH_TO_INSTALL%" (
  mkdir "%PATH_TO_INSTALL%"
)

setx /M PATH "%PATH%;%PATH_TO_INSTALL%"

copy sbbw.exe "%PATH_TO_INSTALL%"
copy sbbw-widget.exe "%PATH_TO_INSTALL%"

echo "[ ✓ ] Sbbw Installed"
sleep 2

echo "[ ✓ ] Done, you can close this"

pause >nul
