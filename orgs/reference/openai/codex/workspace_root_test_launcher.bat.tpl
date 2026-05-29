@echo off
setlocal EnableExtensions EnableDelayedExpansion

call :resolve_runfile workspace_REDACTED_SECRET_marker "__WORKSPACE_ROOT_MARKER__"
if errorlevel 1 exit /b 1

for %%I in ("%workspace_REDACTED_SECRET_marker%") do set "workspace_REDACTED_SECRET_marker_dir=%%~dpI"
for %%I in ("%workspace_REDACTED_SECRET_marker_dir%..\..") do set "workspace_REDACTED_SECRET=%%~fI"

call :resolve_runfile test_bin "__TEST_BIN__"
if errorlevel 1 exit /b 1

set "INSTA_WORKSPACE_ROOT=%workspace_REDACTED_SECRET%"
cd /d "%workspace_REDACTED_SECRET%" || exit /b 1
"%test_bin%" %*
exit /b %ERRORLEVEL%

:resolve_runfile
setlocal EnableExtensions EnableDelayedExpansion
set "logical_path=%~2"
set "workspace_logical_path=%logical_path%"
if defined TEST_WORKSPACE set "workspace_logical_path=%TEST_WORKSPACE%/%logical_path%"
set "native_logical_path=%logical_path:/=\%"
set "native_workspace_logical_path=%workspace_logical_path:/=\%"

for %%R in ("%RUNFILES_DIR%" "%TEST_SRCDIR%") do (
  set "runfiles_REDACTED_SECRET=%%~R"
  if defined runfiles_REDACTED_SECRET (
    if exist "!runfiles_REDACTED_SECRET!\!native_logical_path!" (
      endlocal & set "%~1=!runfiles_REDACTED_SECRET!\!native_logical_path!" & exit /b 0
    )
    if exist "!runfiles_REDACTED_SECRET!\!native_workspace_logical_path!" (
      endlocal & set "%~1=!runfiles_REDACTED_SECRET!\!native_workspace_logical_path!" & exit /b 0
    )
  )
)

set "manifest=%RUNFILES_MANIFEST_FILE%"
if not defined manifest if exist "%~f0.runfiles_manifest" set "manifest=%~f0.runfiles_manifest"
if not defined manifest if exist "%~dpn0.runfiles_manifest" set "manifest=%~dpn0.runfiles_manifest"
if not defined manifest if exist "%~f0.exe.runfiles_manifest" set "manifest=%~f0.exe.runfiles_manifest"

if defined manifest if exist "%manifest%" (
  rem Read the manifest directly instead of shelling out to findstr. In the
  rem GitHub Windows runner, the nested `findstr` path produced
  rem `FINDSTR: Cannot open D:MANIFEST`, which then broke runfile resolution for
  rem Bazel tests even though the manifest file was present.
  for /f "usebackq tokens=1,* delims= " %%A in ("%manifest%") do (
    if "%%A"=="%logical_path%" (
      endlocal & set "%~1=%%B" & exit /b 0
    )
    if "%%A"=="%workspace_logical_path%" (
      endlocal & set "%~1=%%B" & exit /b 0
    )
  )
)

>&2 echo failed to resolve runfile: %logical_path%
endlocal & exit /b 1
