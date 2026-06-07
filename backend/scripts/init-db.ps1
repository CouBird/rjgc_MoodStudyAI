$ErrorActionPreference = "Stop"

if (-not $env:MYSQL_PWD) {
    Write-Error "Set MYSQL_PWD first, for example: `$env:MYSQL_PWD='your-root-password'"
}

$mysql = "C:\Program Files\MySQL\MySQL Server 8.4\bin\mysql.exe"

& $mysql -u root -e "CREATE DATABASE IF NOT EXISTS ai_study_room CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;"

$env:DATABASE_URL = "mysql://root:$env:MYSQL_PWD@127.0.0.1:3306/ai_study_room"
sqlx migrate run
