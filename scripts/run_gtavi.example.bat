@echo off
cd /d "%~dp0.."
if "%~1"=="" (
    echo Usage: %~nx0 "C:\path\to\game_scarlett_master.exe" ["C:\path\to\FidelityFX-SDK\signedbin"]
    exit /b 1
)
set "GAME_EXE=%~1"
set "FFX_DIR=%~2"
if "%FFX_DIR%"=="" set "FFX_DIR=C:\path\to\FidelityFX-SDK\signedbin"
powershell -NoProfile -Command "$env:VICTORY_RELAY='0'; $env:VICTORY_LATE_WAIT='0'; $env:VICTORY_FORCE_60FPS='0'; $env:VICTORY_DISPLAY_MODE='1280x720'; $env:VICTORY_INDIRECT_WINDOW='1'; $env:VICTORY_INPLACE='2'; $env:VICTORY_FFX_DIR='%FFX_DIR%'; $env:VICTORY_FRAMEGEN='1'; $env:VICTORY_AMBIENT='0.6'; $env:VICTORY_TEXTURE_IDLE_MS='20000'; $env:VICTORY_CARD_REHASH_RUNS='1'; $env:VICTORY_WIDE_BACK='1'; $env:VICTORY_WIDE_REFRESH='1'; $env:VICTORY_WIDE_MERGE='1'; $env:VICTORY_JOIN_COPIES='1'; $env:VICTORY_CARD_BUFFERS_MIN_MB='4'; $env:VICTORY_CARD_BUFFERS_MB='2048'; $env:VICTORY_CARD_WINDOWS_MB='1024'; $env:VICTORY_KEEP_SNAPSHOTS='1'; $env:VICTORY_KEEP_SNAPSHOTS_MB='512'; $env:VICTORY_SKIP_PIPELINES='c1eee35dbc50a9ef,f16876929fa64f9f'; .\target\release\victory.exe run $env:GAME_EXE --console series-s --execute"
pause
