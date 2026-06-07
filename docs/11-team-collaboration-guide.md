# 团队协作说明文档

本文档只保留团队分工、接口对接、后续修改位置和协作规则。项目文件结构、技术栈、环境配置、启动命令和检查命令统一维护在 [项目根 README.md](../README.md)。

## 阅读顺序

1. [项目根 README.md](../README.md)：项目概况、技术栈、环境、启动方式和文件结构。
2. [05-restful-api.md](05-restful-api.md)：接口契约、错误码和 DTO 建议。
3. [10-home-room-stats-backend-audit.md](10-home-room-stats-backend-audit.md)：首页、自习室、个人统计当前后端实现状态。
4. 本文档：团队分工、各负责人需要修改的位置和协作规则。

## 当前交付状态

后端核心链路已经打通：

| 页面/模块 | 当前状态 |
| --- | --- |
| 首页 | 已支持当前用户信息、今日学习概览、连续打卡、今日是否打卡、最近情绪 |
| 自习室 | 已支持房间列表、创建、详情、座位、开始学习、心跳、暂停、恢复、结束、休息、超时释放座位 |
| 个人统计 | 已支持周/月/年统计、学习趋势、情绪趋势、主情绪、摘要、环比增长、打卡日历、打卡详情、补卡 |
| 情绪记录 | 已支持情绪标签、评分、备注、模板 AI 反馈、同日打卡关联 |

## 后续需要完善

| 内容 | 当前状态 | 建议负责方向 |
| --- | --- | --- |
| 前端正式页面 | 当前只有 HTML 联调页，需要按原型整理为正式前端页面 | lx【前端负责人】 |
| 首页推荐自习室/最近房间 | 后端没有专用推荐接口，可先用 `/rooms`；如果页面必须有“最近进入”，需要新增接口和表/查询口径 | 前端负责人先确认需求，后端再补 |
| 个人中心页面 | 后端已有统计、打卡、情绪接口，需要前端页面接入并展示 | cy【个人中心负责人】 |
| 用户资料编辑/头像上传 | 当前只实现注册、登录、查看当前用户；资料编辑如昵称、头像上传需要另补接口 | 个人中心负责人 |
| 情绪敏感词过滤 | 目前只限制备注长度，没有敏感词库 | 后端可后续补 |
| 真实 AI 反馈 | 当前是模板反馈，不是真模型调用 | 后端可后续补 |
| 成员加入/离开历史 | 当前只查 active sessions，不记录历史事件 | 自习室增强项 |
| 番茄钟完整状态机 | 后端保存 `mode`，但未做番茄轮次、专注/休息段切换 | 自习室增强项 |

## 前端负责人修改位置

主要关心 `frontend/` 和后端 API 对接。

| 文件 | 用途 |
| --- | --- |
| `frontend/rooms-demo.html` | 自习室联调页，包含登录、房间列表、创建房间、座位、开始学习等逻辑 |
| `frontend/stats-prototype-demo.html` | 按原型风格做的统计页，可参考页面布局和接口调用 |
| `frontend/stats-demo.html` | 原始 JSON 调试页，适合检查接口返回字段 |
| `docs/05-restful-api.md` | 原始 API 文档 |
| `docs/10-home-room-stats-backend-audit.md` | 当前首页、自习室、个人统计后端实现现状 |

前端常用接口：

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

## 个人中心负责人修改位置

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

新增个人资料接口时通常要同步修改：

```text
backend/src/modules/users/routes.rs
backend/src/modules/users/handler.rs
backend/src/modules/users/service.rs
backend/src/modules/users/repository.rs
backend/src/modules/users/dto.rs
backend/tests/auth_api_test.rs 或新增 users_api_test.rs
docs/05-restful-api.md
```

## 关键返回字段

今日统计：

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

个人统计重点字段：

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

打卡详情：

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

## 协作规则

多人修改时建议分支命名：

```text
feature/frontend-pages
feature/profile-center
feature/stats-polish
fix/backend-api
```

修改接口字段时，需要同步更新：

```text
后端 dto.rs
前端调用代码
docs/05-restful-api.md
docs/10-home-room-stats-backend-audit.md
对应 tests/*.rs
```

修改数据库结构时，需要同步更新：

```text
backend/migrations/
backend/src/modules/*/model.rs
backend/src/modules/*/repository.rs
sql/schema.sql
docs/10-home-room-stats-backend-audit.md
```
