$redis = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\taizod1024.redis-windows-fork_Microsoft.Winget.Source_8wekyb3d8bbwe\Redis-8.8.0-Windows-x64-msys2\redis-server.exe"

if (-not (Test-Path $redis)) {
    Write-Error "redis-server.exe was not found at $redis"
}

& $redis
