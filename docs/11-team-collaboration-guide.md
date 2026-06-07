# 团队协作说明文档

本文档用于给同学快速说明项目现状、文件结构、已完成功能、接口对接方式，以及后续主要应该修改哪些文件。

## 项目概况

当前技术栈：

| 部分 | 技术 |
| --- | --- |
| 后端 | Rust、Axum、Tokio、SQLx |
| 数据库 | MySQL |
| 缓存 | Redis，可配置关闭 |
| 鉴权 | JWT Bearer Token |
| 前端测试页 | 原生 HTML/CSS/JS |
| API 前缀 | `http://127.0.0.1:8080/api/v1` |

## 当前仓库结构

当前本地项目根目录为 `D:\rjgc_code`，建议团队仓库根目录也使用这一层，而不是只提交 `backend`。

```text
rjgc_code/
├─ backend/                         Rust 后端项目
│  ├─ config/                        后端配置相关文件
│  ├─ migrations/                    数据库迁移 SQL
│  │  ├─ 202606060001_init_schema.sql
│  │  ├─ 202606060002_add_business_constraints.sql
│  │  ├─ 202606060003_seed_dev_data.sql
│  │  └─ 202606060004_add_stats_indexes.sql
│  ├─ scripts/                       后端辅助脚本，例如迁移脚本
│  ├─ src/
│  │  ├─ ai/                         AI 反馈，目前为模板反馈
│  │  ├─ auth/                       JWT 鉴权提取器
│  │  ├─ cache/                      Redis/缓存封装
│  │  ├─ constants/                  常量，例如有效学习时长、心跳超时
│  │  ├─ db/                         数据库连接与迁移支持
│  │  ├─ modules/                    业务模块
│  │  ├─ routes/                     路由聚合，统一挂到 /api/v1
│  │  ├─ app.rs                      Axum 应用装配
│  │  ├─ config.rs                   环境变量配置读取
│  │  ├─ error.rs                    统一错误类型
│  │  ├─ main.rs                     后端启动入口
│  │  ├─ response.rs                 API 统一响应结构
│  │  └─ state.rs                    应用共享状态
│  ├─ tests/                         后端接口集成测试
│  ├─ .env.example                   环境变量示例
│  ├─ Cargo.toml
│  └─ README.md
├─ docs/                             提交到 Git 的关键 Markdown 文档
│  ├─ 05-restful-api.md
│  ├─ 10-home-room-stats-backend-audit.md
│  └─ 11-team-collaboration-guide.md
├─ frontend/                         简单前端联调页面
│  ├─ rooms-demo.html                自习室联调页面
│  ├─ stats-demo.html                统计接口 JSON 调试页
│  └─ stats-prototype-demo.html      按原型风格做的统计联调页
├─ sql/                              原始 SQL 文件
└─ 原始 Word/HTML/图片资料
```

注意：

- `backend/target`、`backend/target-run`、`backend/target-check`、`backend/target-test` 是 Rust 编译产物，不应该提交到 Git。
- `backend/.env` 是本地环境配置，不应该提交；提交 `backend/.env.example` 即可。

## 后端业务模块结构

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

后续改后端时，优先按这个分层修改，不要把 SQL 写进 `handler.rs`。

## 本轮已经完成的内容

### 首页

| 功能 | 状态 | 主要 API |
| --- | --- | --- |
| 当前用户信息 | 已完成 | `GET /api/v1/users/me` |
| 今日学习概览 | 已完成 | `GET /api/v1/users/me/stats/today` |
| 今日学习分钟数/小时数 | 已完成 | `GET /api/v1/users/me/stats/today` |
| 连续打卡 | 已完成 | `GET /api/v1/users/me/stats/today` |
| 最近情绪 | 已完成 | `GET /api/v1/users/me/stats/today` |
| 补卡后刷新 streak | 已完成 | `POST /api/v1/checkins` |

首页目前没有独立“推荐自习室/最近房间”接口。如果原型需要展示推荐房间，可以先调用 `/rooms` 默认列表。

### 自习室

| 功能 | 状态 | 主要 API |
| --- | --- | --- |
| 自习室列表 | 已完成 | `GET /api/v1/rooms` |
| 过滤过期房间 | 已完成 | `GET /api/v1/rooms` |
| 创建自习室 | 已完成 | `POST /api/v1/rooms` |
| 自习室详情 | 已完成 | `GET /api/v1/rooms/{roomId}` |
| 座位列表 | 已完成 | `GET /api/v1/rooms/{roomId}/seats` |
| 开始学习并占座 | 已完成 | `POST /api/v1/study-sessions` |
| 查询当前活跃学习 | 已完成 | `GET /api/v1/study-sessions/active` |
| 心跳 | 已完成 | `POST /api/v1/study-sessions/{sessionId}/heartbeats` |
| 暂停/恢复/结束学习 | 已完成 | `PATCH /api/v1/study-sessions/{sessionId}` |
| 创建休息 | 已完成 | `POST /api/v1/study-sessions/{sessionId}/breaks` |
| 延长休息 | 已完成 | `PATCH /api/v1/study-breaks/{breakId}` |
| 休息自动结束/恢复 | 已完成，通过查询入口惰性清理 | 房间、座位、活跃学习查询会触发 |
| 心跳超时释放座位 | 已完成，通过查询入口惰性清理 | 超过 10 分钟未心跳会结束会话 |
| 情绪记录 | 已完成 | `POST /api/v1/study-sessions/{sessionId}/emotion-records` |

私密房间不是当前需求，不要求实现密码校验。

### 个人统计

| 功能 | 状态 | 主要 API |
| --- | --- | --- |
| 周/月/年学习统计 | 已完成 | `GET /api/v1/users/me/stats?period=week&date=YYYY-MM-DD` |
| 学习趋势 | 已完成 | `GET /api/v1/users/me/stats` |
| 情绪趋势 | 已完成 | `GET /api/v1/users/me/stats` |
| 独立情绪趋势接口 | 已完成 | `GET /api/v1/users/me/emotion-trends` |
| 主情绪和摘要 | 已完成 | `GET /api/v1/users/me/stats`、`GET /api/v1/users/me/emotion-trends` |
| 打卡日历 | 已完成 | `GET /api/v1/checkins?month=YYYY-MM` |
| 某日打卡详情 | 已完成 | `GET /api/v1/checkins/{date}` |
| 打卡详情展开情绪 | 已完成 | `GET /api/v1/checkins/{date}` |
| 补卡 | 已完成 | `POST /api/v1/checkins` |
| 统计环比增长字段 | 已完成 | `GET /api/v1/users/me/stats` |

## 需要继续完善的内容

| 内容 | 当前状态 | 建议负责方向 |
| --- | --- | --- |
| 前端正式页面 | 当前只有 HTML 联调页，需要按原型整理为正式前端页面 | 李湘【前端负责人】 |
| 首页推荐自习室/最近房间 | 后端没有专用推荐接口，可先用 `/rooms`；如果页面必须有“最近进入”，需要新增接口和表/查询口径 | 前端负责人先确认需求，后端再补 |
| 个人中心页面 | 后端已有统计、打卡、情绪接口，需要前端页面接入并展示 | 楚仪【个人中心负责人】 |
| 用户资料编辑/头像上传 | 当前只实现注册、登录、查看当前用户；资料编辑如昵称、头像上传需要另补接口 | 个人中心负责人 |
| 情绪敏感词过滤 | 目前只限制备注长度，没有敏感词库 | 后端可后续补 |
| 真实 AI 反馈 | 当前是模板反馈，不是真模型调用 | 后端可后续补 |
| 成员加入/离开历史 | 当前只查 active sessions，不记录历史事件 | 自习室增强项 |
| 番茄钟完整状态机 | 后端保存 `mode`，但未做番茄轮次、专注/休息段切换 | 自习室增强项 |

## 主要修改位置

### 前端负责人

主要关心 `frontend/` 和后端 API 对接。

当前可参考文件：

| 文件 | 用途 |
| --- | --- |
| `frontend/rooms-demo.html` | 自习室联调页，包含登录、房间列表、创建房间、座位、开始学习等逻辑 |
| `frontend/stats-prototype-demo.html` | 按原型风格做的统计页，可参考页面布局和接口调用 |
| `frontend/stats-demo.html` | 原始 JSON 调试页，适合检查接口返回字段 |
| `docs/05-restful-api.md` | 原始 API 文档 |
| `docs/10-home-room-stats-backend-audit.md` | 当前首页、自习室、个人统计后端实现现状 |

常用接口：

| 页面 | API |
| --- | --- |
| 登录 | `POST /api/v1/auth/login` |
| 注册 | `POST /api/v1/auth/register` |
| 当前用户 | `GET /api/v1/users/me` |
| 首页今日统计 | `GET /api/v1/users/me/stats/today` |
| 自习室列表 | `GET /api/v1/rooms` |
| 创建自习室 | `POST /api/v1/rooms` |
| 房间详情 | `GET /api/v1/rooms/{roomId}` |
| 座位列表 | `GET /api/v1/rooms/{roomId}/seats` |
| 开始学习 | `POST /api/v1/study-sessions` |
| 当前学习 | `GET /api/v1/study-sessions/active` |
| 结束学习 | `PATCH /api/v1/study-sessions/{sessionId}` |
| 提交情绪 | `POST /api/v1/study-sessions/{sessionId}/emotion-records` |

前端请求需要带登录 token：

```http
Authorization: Bearer <token>
```

### 个人中心负责人

个人中心负责人主要关心个人资料、统计、打卡、情绪记录展示。

后端相关文件：

| 文件 | 用途 |
| --- | --- |
| `backend/src/modules/users/` | 用户注册、登录、当前用户信息；如要做资料编辑，需要在这里新增接口 |
| `backend/src/modules/stats/` | 首页统计、个人统计、周/月/年趋势和环比 |
| `backend/src/modules/checkins/` | 打卡日历、打卡详情、补卡 |
| `backend/src/modules/emotions/` | 情绪记录、独立情绪趋势 |
| `backend/tests/stats_api_test.rs` | 统计接口测试 |
| `backend/tests/checkins_api_test.rs` | 打卡接口测试 |
| `backend/tests/emotions_api_test.rs` | 情绪接口测试 |
| `docs/10-home-room-stats-backend-audit.md` | 首页、自习室、个人统计接口和实现状态说明 |

个人中心常用接口：

| 功能 | API |
| --- | --- |
| 当前用户信息 | `GET /api/v1/users/me` |
| 今日统计 | `GET /api/v1/users/me/stats/today` |
| 个人学习统计 | `GET /api/v1/users/me/stats?period=week\|month\|year&date=YYYY-MM-DD` |
| 独立情绪趋势 | `GET /api/v1/users/me/emotion-trends?period=week\|month\|year&date=YYYY-MM-DD` |
| 打卡日历 | `GET /api/v1/checkins?month=YYYY-MM` |
| 某日打卡详情 | `GET /api/v1/checkins/{date}` |
| 补卡 | `POST /api/v1/checkins` |

如果要新增“编辑个人资料”，建议新增：

| 建议 API | 说明 | 建议后端位置 |
| --- | --- | --- |
| `PATCH /api/v1/users/me` | 修改昵称、头像地址等基础资料 | `backend/src/modules/users/` |
| `POST /api/v1/users/me/avatar` | 上传头像，如果需要文件上传 | `backend/src/modules/users/`、`backend/src/storage/` |

新增接口时要同步修改：

```text
backend/src/modules/users/routes.rs
backend/src/modules/users/handler.rs
backend/src/modules/users/service.rs
backend/src/modules/users/repository.rs
backend/src/modules/users/dto.rs
backend/tests/auth_api_test.rs 或新增 users_api_test.rs
docs/05-restful-api.md
```

## 关键 API 返回字段

### 今日统计

`GET /api/v1/users/me/stats/today`

```json
{
  "todayMinutes": 120,
  "todayHours": 2.0,
  "streakDays": 5,
  "todayCheckin": true,
  "validCheckin": true,
  "latestEmotion": "平静",
  "mood": "平静"
}
```

### 个人统计

`GET /api/v1/users/me/stats?period=week&date=2026-06-07`

重点字段：

```json
{
  "period": "week",
  "startDate": "2026-06-01",
  "endDate": "2026-06-07",
  "totalMinutes": 1710,
  "totalHours": 28.5,
  "previousTotalMinutes": 1500,
  "totalMinutesChange": 210,
  "totalHoursGrowthPercent": 14.0,
  "averageDailyMinutes": 244,
  "averageDailyHours": 4.1,
  "validSessionCount": 12,
  "checkinCount": 7,
  "studyDays": 7,
  "streakDays": 7,
  "studyTrend": [],
  "emotionTrend": [],
  "emotionMap": {},
  "mainEmotion": "平静",
  "summary": "这段时间内你常感到平静"
}
```

### 打卡详情

`GET /api/v1/checkins/{date}`

```json
{
  "checkinId": "1",
  "date": "2026-06-07",
  "totalMinutes": 120,
  "isMakeup": false,
  "makeupReason": null,
  "summaryNote": "完成阅读任务",
  "emotionRecordId": "9",
  "emotionRecord": {
    "emotionRecordId": "9",
    "emotionTag": "平静",
    "emotionScore": 7,
    "userNote": "状态稳定",
    "aiFeedback": "继续保持当前节奏。",
    "createdAt": "2026-06-07T10:30:00Z"
  },
  "createdAt": "2026-06-07T10:30:00Z"
}
```

## 本地运行方式

后端：

```powershell
cd D:\rjgc_code\backend
Copy-Item .env.example .env
.\scripts\migrate.ps1
cargo run --target-dir target-run
```

如果出现端口占用：

```text
Error: Io(Os { code: 10048, kind: AddrInUse, message: "通常每个套接字地址只允许使用一次。" })
```

说明 `127.0.0.1:8080` 已经有后端或其他程序在运行。解决方式是关闭旧进程，或者修改 `backend/.env` 里的 `APP_PORT`。

前端测试页：

```text
D:\rjgc_code\frontend\rooms-demo.html
D:\rjgc_code\frontend\stats-prototype-demo.html
D:\rjgc_code\frontend\stats-demo.html
```

这些页面可以直接用浏览器打开，默认请求 `http://127.0.0.1:8080/api/v1`。

后端检查命令：

```powershell
cd D:\rjgc_code\backend
cargo fmt
cargo check --target-dir target-check
cargo test --target-dir target-test
```

## Git 协作建议

建议团队统一以项目根目录作为仓库根：

```text
rjgc_MoodStudyAI/
├─ backend/
├─ frontend/
├─ docs/
└─ sql/
```

建议不要提交：

```text
backend/.env
backend/target/
backend/target-run/
backend/target-check/
backend/target-test/
```

建议提交：

```text
backend/.env.example
backend/migrations/
backend/src/
backend/tests/
frontend/
docs/
sql/
```

多人修改时建议分支命名：

```text
feature/frontend-pages
feature/profile-center
feature/stats-polish
fix/backend-api
```

每次修改接口字段时，需要同步更新：

```text
后端 dto.rs
前端调用代码
docs/05-restful-api.md
docs/10-home-room-stats-backend-audit.md
对应 tests/*.rs
```

## 阅读顺序

新同学接手时建议按这个顺序阅读：

1. [项目根 README.md](../README.md)：项目概览、运行方式和提交范围。
2. [05-restful-api.md](05-restful-api.md)：接口契约。
3. [10-home-room-stats-backend-audit.md](10-home-room-stats-backend-audit.md)：首页、自习室、个人统计当前实现状态。
4. 本文档：团队分工、项目结构和修改位置。
