# AI 情绪治愈自习打卡空间

本仓库是“AI 情绪治愈自习打卡空间”的团队协作代码仓库，包含 Rust 后端、简单前端联调页面、数据库脚本和必要项目文档。

## 当前内容

| 目录 | 说明 |
| --- | --- |
| `backend/` | Rust + Axum 后端服务，包含登录、自习室、学习会话、打卡、情绪、统计等模块 |
| `frontend/` | 简单 HTML 联调页面，用于测试自习室和个人统计接口 |
| `sql/` | 原始数据库建表和初始化数据 |
| `docs/` | 只保留关键协作文档：API 文档、后端实现核查、团队协作说明 |

## 已完成功能

- 用户注册、登录、JWT 鉴权、当前用户信息。
- 首页今日学习统计、连续打卡、最近情绪。
- 自习室列表、创建、详情、座位、开始学习、心跳、暂停、恢复、结束。
- 休息创建、休息延长、休息自动恢复、心跳超时释放座位。
- 有效学习自动打卡、补卡、补卡后刷新连续打卡天数。
- 情绪记录、模板 AI 反馈、打卡详情展开情绪内容。
- 个人统计周/月/年聚合、学习趋势、情绪趋势、主情绪、摘要和环比增长字段。

## 本地运行

后端：

```powershell
cd backend
Copy-Item .env.example .env
.\scripts\migrate.ps1
cargo run --target-dir target-run
```

前端测试页可直接用浏览器打开：

```text
frontend/rooms-demo.html
frontend/stats-prototype-demo.html
frontend/stats-demo.html
```

默认后端地址：

```text
http://127.0.0.1:8080/api/v1
```

如 `cargo run` 报端口占用，说明 `127.0.0.1:8080` 已有服务运行，可以关闭旧进程或修改 `backend/.env` 中的 `APP_PORT`。

## 提交范围

本仓库不提交 Word 原始文档、转换产物、图片资料、本地 `.env` 和 Rust 编译产物。Markdown 文档只保留：

- `docs/05-restful-api.md`
- `docs/10-home-room-stats-backend-audit.md`
- `docs/11-team-collaboration-guide.md`
- `README.md`

## 重要文档

- [RESTful API 文档](docs/05-restful-api.md)
- [首页、自习室、个人统计后端实现核查](docs/10-home-room-stats-backend-audit.md)
- [团队协作说明文档](docs/11-team-collaboration-guide.md)
