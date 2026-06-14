# AI 情绪治愈自习打卡空间

本仓库是“AI 情绪治愈自习打卡空间”的团队协作代码仓库，包含 Rust 后端、MySQL 数据库脚本、简单前端联调页面和关键项目文档。

项目目标：提供一个带自习室、学习计时、自动打卡、补卡、情绪记录和个人学习统计的学习空间。用户登录后可以进入自习室占座学习，学习满有效时长后自动生成打卡记录，也可以在 7 日内补卡；个人统计页支持按周/月/年查看学习趋势、情绪趋势、主情绪摘要和环比增长。

## 已完成功能

| 模块 | 已完成功能 |
| --- | --- |
| 用户 | 注册、登录、JWT 鉴权、获取当前用户信息 |
| 首页 | 今日学习分钟数/小时数、连续打卡、今日是否打卡、最近情绪 |
| 自习室 | 房间列表、创建房间、房间详情、座位列表、占座开始学习 |
| 学习会话 | 当前活跃学习、心跳、暂停、恢复、结束学习、有效学习自动打卡 |
| 休息 | 创建休息、延长休息、休息到期自动恢复学习 |
| 超时处理 | 心跳超过 10 分钟后自动结束会话、释放座位，满 10 分钟则生成有效打卡 |
| 打卡 | 打卡日历、某日打卡详情、7 日内补卡、补卡后刷新连续打卡天数 |
| 情绪 | 提交情绪标签/评分/备注、模板 AI 反馈、同日打卡关联情绪记录 |
| 个人统计 | 周/月/年学习统计、学习趋势、情绪趋势、主情绪、摘要、环比增长字段 |

仍可增强的内容包括：正式前端页面、个人资料编辑/头像上传、敏感词过滤、真实 AI 反馈、成员加入/离开历史、番茄钟完整状态机。

## 技术栈

| 部分 | 技术 |
| --- | --- |
| 后端语言 | Rust 1.95.0 |
| Web 框架 | Axum 0.8 |
| 异步运行时 | Tokio |
| 数据库 | MySQL 8.0+ |
| SQL/迁移 | SQLx |
| 缓存 | Redis，可通过环境变量关闭 |
| 鉴权 | JWT Bearer Token |
| 密码哈希 | bcrypt |
| 日志/中间件 | tracing、tower-http |
| 前端联调页 | 原生 HTML/CSS/JavaScript |

## 环境要求

本地开发建议准备：

| 工具 | 说明 |
| --- | --- |
| Rust | 仓库内 `backend/rust-toolchain.toml` 指定 `1.95.0` |
| MySQL | 需要运行迁移和真实接口联调时启用 |
| SQLx CLI | 执行 `backend/scripts/migrate.ps1` 需要 |
| Redis | 可选；默认可关闭 |
| PowerShell | 当前脚本按 Windows PowerShell 编写 |
| 浏览器 | 直接打开 `frontend/*.html` 联调页面 |

安装 SQLx CLI 示例：

```powershell
cargo install sqlx-cli --no-default-features --features mysql,rustls
```

## 环境配置

复制后端环境变量示例：

```powershell
cd backend
Copy-Item .env.example .env
```

`backend/.env.example` 默认配置：

```env
APP_HOST=127.0.0.1
APP_PORT=8080

DATABASE_ENABLED=false
DATABASE_URL=mysql://root:password@127.0.0.1:3306/ai_study_room
DATABASE_MAX_CONNECTIONS=5

REDIS_ENABLED=false
REDIS_URL=redis://127.0.0.1:6379

JWT_SECRET=change-me-in-development
JWT_USER_EXPIRE_HOURS=24
JWT_ADMIN_EXPIRE_HOURS=8

AVATAR_DIR=storage/avatars
MAX_AVATAR_BYTES=3145728
```

如果要联调完整业务接口，需要把 `DATABASE_URL` 改成自己的 MySQL 账号密码，并把 `DATABASE_ENABLED=true`。Redis 不是必需项，不启用时保持 `REDIS_ENABLED=false` 即可。

## 启动项目

### 1. 初始化数据库

确认 MySQL 已启动，且 `backend/.env` 中 `DATABASE_URL` 正确，然后执行：

```powershell
cd backend
.\scripts\migrate.ps1
```

该脚本会执行：

```powershell
sqlx database create
sqlx migrate run
```

### 2. 启动后端

```powershell
cd backend
cargo run --target-dir target-run
```

默认服务地址：

```text
http://127.0.0.1:8080
```

API 前缀：

```text
http://127.0.0.1:8080/api/v1
```

健康检查：

```text
GET /health
GET /api/v1/health
```

如果启动时报端口占用：

```text
Error: Io(Os { code: 10048, kind: AddrInUse, message: "通常每个套接字地址只允许使用一次。" })
```

说明 `127.0.0.1:8080` 已经有后端或其他程序在运行。关闭旧进程，或修改 `backend/.env` 中的 `APP_PORT`。

### 3. 打开前端联调页

这些页面可以直接用浏览器打开：

```text
frontend/rooms-demo.html
frontend/stats-prototype-demo.html
frontend/stats-demo.html
```

默认请求地址为：

```text
http://127.0.0.1:8080/api/v1
```

## 检查命令

后端格式化：

```powershell
cd backend
cargo fmt
```

后端编译检查：

```powershell
cd backend
cargo check --target-dir target-check
```

后端测试：

```powershell
cd backend
cargo test --target-dir target-test
```

## 项目文件结构

```text
rjgc_MoodStudyAI/
├─ backend/                         Rust 后端项目
│  ├─ config/                        后端配置文件
│  ├─ migrations/                    SQLx 数据库迁移
│  │  ├─ 202606060001_init_schema.sql
│  │  ├─ 202606060002_add_business_constraints.sql
│  │  ├─ 202606060003_seed_dev_data.sql
│  │  └─ 202606060004_add_stats_indexes.sql
│  ├─ scripts/                       开发脚本
│  ├─ src/
│  │  ├─ ai/                         模板 AI 反馈
│  │  ├─ auth/                       JWT、鉴权提取器、密码哈希
│  │  ├─ cache/                      Redis/登录锁定扩展位置
│  │  ├─ constants/                  角色、状态、业务限制常量
│  │  ├─ db/                         数据库连接、健康检查、迁移支持
│  │  ├─ modules/                    业务模块
│  │  ├─ routes/                     路由聚合，统一挂到 /api/v1
│  │  ├─ app.rs                      Axum 应用装配
│  │  ├─ config.rs                   环境变量读取
│  │  ├─ error.rs                    统一错误类型
│  │  ├─ main.rs                     后端启动入口
│  │  ├─ response.rs                 API 统一响应结构
│  │  └─ state.rs                    应用共享状态
│  ├─ tests/                         接口集成测试
│  ├─ .env.example                   环境变量示例
│  ├─ Cargo.toml
│  └─ rust-toolchain.toml
├─ docs/                             关键 Markdown 文档
│  ├─ 05-restful-api.md
│  ├─ 10-home-room-stats-backend-audit.md
│  └─ 11-team-collaboration-guide.md
├─ frontend/                         前端联调页面
│  ├─ rooms-demo.html
│  ├─ stats-demo.html
│  └─ stats-prototype-demo.html
├─ sql/                              原始数据库脚本
│  ├─ data.sql
│  └─ schema.sql
├─ .gitignore
└─ README.md
```

## 后端业务模块

`backend/src/modules/` 是主要业务代码目录：

```text
backend/src/modules/
├─ admin/              管理员登录
├─ checkins/           打卡日历、打卡详情、补卡
├─ emotions/           情绪记录、情绪趋势
├─ rooms/              自习室列表、创建、详情、座位
├─ stats/              首页今日统计、个人学习统计
├─ study_breaks/       休息创建、休息延长
├─ study_sessions/     开始学习、暂停、恢复、结束、心跳、超时清理
└─ users/              注册、登录、当前用户信息
```

每个业务模块基本按同一结构组织：

| 文件 | 作用 |
| --- | --- |
| `routes.rs` | 注册该模块 API 路径 |
| `handler.rs` | 处理 HTTP 请求、解析参数、返回响应 |
| `service.rs` | 业务规则、状态机、事务编排 |
| `repository.rs` | SQL 查询和数据库写入 |
| `dto.rs` | 请求/响应 JSON 字段 |
| `model.rs` | 数据库查询结果结构 |
| `mod.rs` | 模块导出 |

## 常用 API

| 功能 | API |
| --- | --- |
| 注册 | `POST /api/v1/auth/register` |
| 登录 | `POST /api/v1/auth/login` |
| 当前用户 | `GET /api/v1/users/me` |
| 今日统计 | `GET /api/v1/users/me/stats/today` |
| 自习室列表 | `GET /api/v1/rooms` |
| 创建自习室 | `POST /api/v1/rooms` |
| 房间详情 | `GET /api/v1/rooms/{roomId}` |
| 座位列表 | `GET /api/v1/rooms/{roomId}/seats` |
| 开始学习 | `POST /api/v1/study-sessions` |
| 当前学习 | `GET /api/v1/study-sessions/active` |
| 结束学习 | `PATCH /api/v1/study-sessions/{sessionId}` |
| 创建休息 | `POST /api/v1/study-sessions/{sessionId}/breaks` |
| 提交情绪 | `POST /api/v1/study-sessions/{sessionId}/emotion-records` |
| 个人统计 | `GET /api/v1/users/me/stats?period=week\|month\|year&date=YYYY-MM-DD` |
| 情绪趋势 | `GET /api/v1/users/me/emotion-trends?period=week\|month\|year&date=YYYY-MM-DD` |
| 打卡日历 | `GET /api/v1/checkins?month=YYYY-MM` |
| 打卡详情 | `GET /api/v1/checkins/{date}` |
| 补卡 | `POST /api/v1/checkins` |

登录后请求需要带 token：

```http
Authorization: Bearer <token>
```

## Git 提交范围

本仓库不提交：

```text
*.docx
*.doc
_extracted_docx/
backend/.env
backend/target/
backend/target-run/
backend/target-check/
backend/target-test/
backend/storage/
```

Markdown 文档只保留：

```text
README.md
docs/05-restful-api.md
docs/10-home-room-stats-backend-audit.md
docs/11-team-collaboration-guide.md
```

## 重要文档

- [RESTful API 文档](docs/05-restful-api.md)
- [首页、自习室、个人统计后端实现核查](docs/10-home-room-stats-backend-audit.md)
- [团队协作说明文档](docs/11-team-collaboration-guide.md)

## 2026-06-10 当前补充说明

本节为当前开发状态的追加说明，不替代上文原有启动和协作说明。

### 当前新增完成内容

| 模块 | 当前补充状态 |
| --- | --- |
| 个人中心后端 | 已补齐 `PATCH /api/v1/users/me`、`POST /api/v1/users/me/avatar`、`PATCH /api/v1/users/me/password` |
| 当前用户信息 | `GET /api/v1/users/me` 已返回 `profile`、`streakDays`，可直接支撑个人中心资料卡 |
| 头像上传 | 支持 multipart 字段 `file`，限制 JPG/PNG 和 `MAX_AVATAR_BYTES`，保存到 `AVATAR_DIR` |
| 静态头像访问 | 后端已挂载 `/storage/avatars/...`，前端从本地 HTML 打开时会拼接 `http://127.0.0.1:8080` |
| 个人中心前端联调 | `frontend/rooms-demo.html` 的“个人中心”不再是占位弹窗，已接入查看资料、保存资料、上传头像、修改密码 |
| 文档 | `docs/05-restful-api.md` 已追加个人资料、头像上传、修改密码的返回字段示例 |

### Windows 本地启动补充

如果 Rust 安装到了 D 盘，例如：

```text
D:\Rust\.cargo
D:\Rust\.rustup
```

每次打开新的 PowerShell/Cursor 终端后，可以先执行：

```powershell
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"
$env:Path = "D:\Rust\.cargo\bin;$env:Path"
```

然后再进入后端启动：

```powershell
cd d:\rjgc_MoodStudyAI\backend
cargo run --target-dir target-run
```

如果已经编译过且没有改 Rust 代码，也可以直接运行已生成的可执行文件：

```powershell
cd d:\rjgc_MoodStudyAI\backend
.\target-run\debug\backend.exe
```

### 手动初始化数据库补充

如果不想安装 `sqlx-cli`，本地联调可以直接用 MySQL 执行原始 SQL：

```sql
CREATE DATABASE IF NOT EXISTS ai_study_room DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE ai_study_room;
SOURCE D:/rjgc_MoodStudyAI/sql/schema.sql;
SOURCE D:/rjgc_MoodStudyAI/sql/data.sql;
```

对应 `.env` 至少需要：

```env
DATABASE_ENABLED=true
DATABASE_URL=mysql://root:你的MySQL密码@127.0.0.1:3306/ai_study_room
REDIS_ENABLED=false
AVATAR_DIR=storage/avatars
MAX_AVATAR_BYTES=3145728
```

MySQL 服务名在当前本地环境中可能是 `mysql`，可用以下命令启动：

```powershell
net start mysql
```

如果服务名不同，可用 `services.msc` 或 `sc query state= all | findstr /I mysql` 查看实际服务名。

### 当前联调入口

后端启动成功后，终端会看到：

```text
backend listening on http://127.0.0.1:8080
```

健康检查：

```text
http://127.0.0.1:8080/health
```

前端联调页面：

```text
d:\rjgc_MoodStudyAI\frontend\rooms-demo.html
```

页面中可测试：首页、自习室、学习统计入口、个人中心资料编辑、头像上传和密码修改。后端运行期间不要关闭 `cargo run` 所在终端。
