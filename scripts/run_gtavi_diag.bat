@echo off
cd /d "C:\Users\andre\Desktop\victory-fork"
title GTA VI - VIctory PROFILER
powershell -NoProfile -Command "$env:VICTORY_RELAY='0'; $env:VICTORY_FORCE_60FPS='0'; $env:VICTORY_DISPLAY_MODE='1280x720'; $env:VICTORY_D3D12_DRED='1'; $env:VICTORY_INDIRECT_WINDOW='1'; $env:VICTORY_INPLACE='2'; $env:VICTORY_CARD_REHASH_RUNS='1'; $env:VICTORY_WIDE_BACK='1'; $env:VICTORY_WIDE_REFRESH='1'; $env:VICTORY_WIDE_MERGE='1'; $env:VICTORY_JOIN_COPIES='1'; $env:VICTORY_CARD_BUFFERS_MIN_MB='16'; $env:VICTORY_CARD_BUFFERS_MB='2048'; $env:VICTORY_CARD_WINDOWS_MB='512'; $env:VICTORY_KEEP_SNAPSHOTS='1'; $env:VICTORY_KEEP_SNAPSHOTS_MB='512'; $env:VICTORY_GPU_TIMING='1'; $env:VICTORY_INPLACE_PROBE='1'; $env:VICTORY_SKIP_PIPELINES='c1eee35dbc50a9ef,f16876929fa64f9f'; .\target\diag\victory.exe run 'C:\Users\andre\Downloads\game.Xbox.v1.0.2.1\game_scarlett_master.exe' --execute 2>&1 | Tee-Object -FilePath 'C:\Users\andre\Desktop\gtavi_diag.log'"
pause
