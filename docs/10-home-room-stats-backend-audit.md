# 首页、自习室、个人统计后端实现核查

本文整理【首页、自习室、个人统计】三个页面当前已经实现的后端逻辑、对应 API、数据库字段，以及本轮补齐后仍保留的后端缺口。当前口径已按已实现代码、需求说明书、API 文档和数据库文档重新同步。

## 首页

### 已实现后端逻辑

| 功能 | API | 后端实现 | 数据库字段 |
| --- | --- | --- | --- |
| 登录后今日学习概览 | `GET /api/v1/users/me/stats/today` | 已实现。返回今日学习分钟数/小时数、连续打卡、今日是否打卡、最近情绪 | `checkin_records.user_id`、`checkin_records.checkin_date`、`checkin_records.total_minutes`、`emotion_records.emotion_tag`、`study_sessions.user_id` |
| 连续打卡 | `GET /api/v1/users/me/stats/today` | 已实现为倒序扫描 `checkin_records.checkin_date`，不是只信 `users.streak_days` 缓存 | `checkin_records.checkin_date`、`users.streak_days` |
| 补卡后 streak 缓存刷新 | `POST /api/v1/checkins` | 已实现。补卡成功后在事务内重算连续打卡，并更新 `users.streak_days` | `checkin_records.checkin_date`、`checkin_records.is_makeup`、`users.streak_days` |
| 今日情绪/最近情绪 | `GET /api/v1/users/me/stats/today` | 已实现。按当前用户最新 `emotion_records.created_at` 返回 | `emotion_records.emotion_tag`、`emotion_records.created_at`、`study_sessions.user_id` |
| 当前用户信息 | `GET /api/v1/users/me` | 已实现，用于前端显示昵称和头像 | `users.id`、`users.nickname`、`users.avatar_url`、`users.phone`、`users.status` |

### 主要源码

| 文件 | 说明 |
| --- | --- |
| `backend/src/modules/stats/service.rs` | 首页今日统计业务逻辑 |
| `backend/src/modules/stats/repository.rs` | 首页统计 SQL 查询 |
| `backend/src/modules/checkins/service.rs` | 补卡和 streak 刷新逻辑 |
| `backend/src/modules/checkins/repository.rs` | 打卡写入、打卡聚合、连续打卡重算 |
| `backend/src/modules/users/service.rs` | 当前用户信息 |

### 仍未实现或可增强

| 缺口 | 说明 |
| --- | --- |
| 首页推荐自习室/最近房间专用接口 | 需求文档没有强制定义专用推荐接口；前端原型如果需要推荐房间，可先使用 `/rooms` 默认列表代替。当前没有独立的“推荐房间”“最近进入房间”接口 |

## 自习室

### 已实现后端逻辑

| 功能 | API | 后端实现 | 数据库字段 |
| --- | --- | --- | --- |
| 自习室列表 | `GET /api/v1/rooms?keyword=&status=&page=&pageSize=` | 已实现分页、关键词、状态过滤；未传 `status` 时默认只返回 `status='open'` 且 `close_at > NOW()` 的可进入房间；`status=open` 也会过滤过期房间；返回当前人数、容量、房主、开放时间 | `study_rooms.*`、`users.nickname`、`users.avatar_url`、`study_sessions.status` |
| 创建自习室 | `POST /api/v1/rooms` | 已实现。校验名称、容量、描述、`closeAt` 至少晚于当前 1 小时；自动创建座位。私密房间不是当前产品需求，密码校验不作为必做项 | `study_rooms.name`、`study_rooms.capacity`、`study_rooms.is_private`、`study_rooms.password`、`study_rooms.creator_id`、`study_rooms.close_at`、`study_room_seats.*` |
| 自习室详情 | `GET /api/v1/rooms/{roomId}` | 已实现。访问前会做惰性清理，返回房间、座位列表、当前在线成员 | `study_rooms`、`study_room_seats`、`study_sessions`、`users` |
| 座位列表 | `GET /api/v1/rooms/{roomId}/seats` | 已实现。访问前会做惰性清理，返回座位状态和占用用户 | `study_room_seats.status`、`study_sessions.status`、`users` |
| 开始学习 | `POST /api/v1/study-sessions` | 已实现事务：校验用户无活跃会话、房间 open、未过期、未满、座位可用，然后占座并创建会话 | `study_sessions`、`study_room_seats.status`、`study_rooms.status`、`study_rooms.close_at` |
| 当前活跃学习 | `GET /api/v1/study-sessions/active` | 已实现。查询前会执行惰性清理，过期休息会恢复学习，心跳超时会结束会话并释放座位 | `study_sessions.status IN ('studying','paused','resting')`、`study_sessions.last_heartbeat_at` |
| 心跳 | `POST /api/v1/study-sessions/{sessionId}/heartbeats` | 已实现。更新 `last_heartbeat_at`；超时判定阈值为 10 分钟，由后续活跃查询、房间查询等入口触发清理 | `study_sessions.last_heartbeat_at` |
| 心跳超时释放座位 | 惰性清理触发于房间/座位/活跃会话等查询入口 | 已实现。`studying`、`paused` 会话超过心跳阈值后自动结束；结束时计算学习时长、释放座位，满 10 分钟则自动打卡并刷新 streak | `study_sessions.status`、`study_sessions.end_time`、`study_sessions.duration_minutes`、`study_sessions.is_valid`、`study_room_seats.status`、`checkin_records`、`users.streak_days` |
| 暂停/恢复/结束学习 | `PATCH /api/v1/study-sessions/{sessionId}` | 已实现状态机校验；结束时计算时长，释放座位；满 10 分钟自动打卡；若会话已有情绪记录，会关联到同日打卡 | `study_sessions.status`、`study_sessions.end_time`、`study_sessions.duration_minutes`、`study_sessions.is_valid`、`study_room_seats.status`、`checkin_records.emotion_record_id` |
| 创建休息 | `POST /api/v1/study-sessions/{sessionId}/breaks` | 已实现。仅 `studying` 可进入 `resting` | `study_breaks`、`study_sessions.status` |
| 延长休息 | `PATCH /api/v1/study-breaks/{breakId}` | 已实现。增加休息时长，标记 extended | `study_breaks.duration`、`study_breaks.is_extended` |
| 休息自动结束/恢复 | 惰性清理触发于房间/座位/活跃会话等查询入口 | 已实现。到达 `start_time + duration` 后自动设置 `study_breaks.end_time`，并把会话恢复为 `studying` | `study_breaks.start_time`、`study_breaks.duration`、`study_breaks.end_time`、`study_sessions.status`、`study_sessions.last_heartbeat_at` |
| 情绪打卡 | `POST /api/v1/study-sessions/{sessionId}/emotion-records` | 已实现。校验会话归属和同一 session 不重复；保存情绪标签、评分、备注和模板 AI 反馈；若同日已有打卡会关联 `emotion_record_id` | `emotion_records.*`、`checkin_records.emotion_record_id` |

### 主要源码

| 文件 | 说明 |
| --- | --- |
| `backend/src/modules/rooms/service.rs` | 自习室列表、创建、详情、座位业务逻辑，查询入口触发惰性清理 |
| `backend/src/modules/rooms/repository.rs` | 自习室和座位 SQL，默认过滤可进入房间 |
| `backend/src/modules/study_sessions/service.rs` | 学习会话业务逻辑 |
| `backend/src/modules/study_sessions/repository.rs` | 占座、开始学习、结束学习、心跳、休息/超时清理事务 |
| `backend/src/modules/study_breaks/service.rs` | 休息业务逻辑 |
| `backend/src/modules/study_breaks/repository.rs` | 创建休息、延长休息事务 |
| `backend/src/constants/limits.rs` | 有效学习最小时长、心跳超时阈值等常量 |

### 仍未实现或可增强

| 缺口 | 说明 |
| --- | --- |
| 私密房间密码校验 | 不实现。【==ai自己实现了，后续看情况删掉==】已按产品口径明确“不需要私密房间”，因此不作为后端缺口处理。数据库中保留 `is_private/password` 字段只是兼容原始表结构 |
| 正向计时/番茄钟模式的完整计时器逻辑 | 后端目前保存 `study_sessions.mode`，可区分模式字符串；但没有实现番茄钟轮次、专注/休息段切换、模式互换等专门状态机。 |
| 房间成员历史/在线状态更细粒度 | 当前成员来自 active sessions，只能表达“当前在房间中”。没有单独记录用户加入、离开、临时掉线、重连等历史事件 |
| 情绪入口未强制 session 必须 ended | `POST /emotion-records` 只校验 session 属于用户且未重复提交，未强制学习会话必须已经结束。当前实现允许学习中先记录情绪，结束后再关联到同日打卡 |

## 个人统计

### 已实现后端逻辑

| 功能 | API | 后端实现 | 数据库字段 |
| --- | --- | --- | --- |
| 学习统计 | `GET /api/v1/users/me/stats?period=week\|month\|year&date=YYYY-MM-DD` | 已实现自然周/月/年区间，返回总时长、日均、学习天数、连续打卡、学习趋势、情绪趋势、主情绪、摘要 | `checkin_records`、`study_sessions`、`emotion_records` |
| 统计环比增长 | `GET /api/v1/users/me/stats` | 已实现。按上一自然周/月/年计算上一周期值、差值和增长百分比，覆盖总学习时长、日均学习、有效会话数、打卡数、学习天数 | `checkin_records.total_minutes`、`checkin_records.checkin_date`、`study_sessions.end_time`、`study_sessions.is_valid` |
| 学习趋势 | `GET /api/v1/users/me/stats` | 已实现。按周期补齐每一天，缺失日期返回 0 | `checkin_records.checkin_date`、`checkin_records.total_minutes` |
| 完成打卡/学习天数 | `GET /api/v1/users/me/stats` | 已实现。来自周期内 `checkin_records` | `checkin_records.user_id`、`checkin_records.checkin_date` |
| 有效会话数 | `GET /api/v1/users/me/stats` | 已实现。统计 `ended` 且 `is_valid=1` | `study_sessions.status`、`study_sessions.is_valid`、`study_sessions.end_time` |
| 情绪趋势 | `GET /api/v1/users/me/stats` | 已实现文档要求的 `emotionMap`、`emotionTrend`、`mainEmotion`、`summary` | `emotion_records.emotion_tag`、`emotion_records.created_at`、`study_sessions.user_id` |
| 独立情绪趋势 | `GET /api/v1/users/me/emotion-trends?period=week\|month\|year&date=YYYY-MM-DD` | 已实现。支持 `date` 基准日期和自然周/月/年；返回 `emotionMap`、`items`、`mainEmotion`、`summary`，并兼容旧字段 `trends`、`tagDistribution` | `emotion_records.emotion_tag`、`emotion_records.emotion_score`、`emotion_records.created_at`、`study_sessions.user_id` |
| 打卡日历 | `GET /api/v1/checkins?month=YYYY-MM` | 已实现。返回整月每日 `checkedIn`、`totalMinutes`、`isMakeup` | `checkin_records` |
| 某日打卡详情 | `GET /api/v1/checkins/{date}` | 已实现。返回打卡基础信息，并展开 `emotionRecord`：情绪标签、评分、备注、AI 反馈、创建时间 | `checkin_records`、`emotion_records` |
| 补卡 | `POST /api/v1/checkins` | 已实现 7 日内补卡、不能未来日期、不能重复日期；补卡后重算并更新 `users.streak_days` | `checkin_records`、`users.streak_days` |
| 情绪打卡 | `POST /api/v1/study-sessions/{sessionId}/emotion-records` | 已实现情绪标签、评分、备注、AI 模板反馈、同一 session 唯一；会尽量关联同日打卡 | `emotion_records`、`study_sessions`、`checkin_records.emotion_record_id` |

### 主要源码

| 文件 | 说明 |
| --- | --- |
| `backend/src/modules/stats/service.rs` | 个人统计聚合逻辑、自然周期计算、环比增长计算 |
| `backend/src/modules/stats/dto.rs` | 个人统计响应字段 |
| `backend/src/modules/stats/repository.rs` | 统计 SQL |
| `backend/src/modules/checkins/service.rs` | 打卡日历、详情、补卡 |
| `backend/src/modules/checkins/repository.rs` | 打卡 SQL、自动打卡 upsert、连续打卡重算、情绪关联 |
| `backend/src/modules/emotions/service.rs` | 情绪打卡、独立情绪趋势 |
| `backend/src/modules/emotions/repository.rs` | 情绪 SQL |

### 仍未实现或可增强

| 缺口 | 说明 |
| --- | --- |
| 情绪敏感词过滤 | 需求提到用户输入需敏感词过滤；当前仅限制备注长度，没有敏感词词库和过滤策略 |
| AI 情绪反馈是真模板，不是真 AI 调用 | 当前 `ai::feedback::generate_template_feedback` 是模板反馈，满足基础演示和联调，但不是模型调用 |

## 数据库字段对照

| 表 | 页面 | 关键字段 |
| --- | --- | --- |
| `users` | 首页、自习室、统计 | `id`、`nickname`、`phone`、`avatar_url`、`status`、`streak_days` |
| `study_rooms` | 自习室 | `id`、`name`、`description`、`capacity`、`is_private`、`password`、`status`、`creator_id`、`open_at`、`close_at` |
| `study_room_seats` | 自习室 | `id`、`room_id`、`seat_code`、`status` |
| `study_sessions` | 自习室、首页、统计 | `id`、`room_id`、`user_id`、`seat_id`、`mode`、`study_content`、`start_time`、`end_time`、`duration_minutes`、`is_valid`、`status`、`last_heartbeat_at` |
| `study_breaks` | 自习室 | `id`、`session_id`、`start_time`、`end_time`、`duration`、`is_extended` |
| `checkin_records` | 首页、统计 | `id`、`user_id`、`checkin_date`、`emotion_record_id`、`total_minutes`、`is_makeup`、`makeup_reason`、`summary_note`、`created_at` |
| `emotion_records` | 首页、统计 | `id`、`session_id`、`emotion_tag`、`emotion_score`、`user_note`、`ai_feedback`、`created_at` |

三个页面的核心后端链路已经打通：

- 登录后首页概览、连续打卡、补卡后 streak 刷新。
- 自习室列表、创建、详情、座位、开始学习、心跳、休息、休息自动恢复、心跳超时释放座位、结束学习。
- 有效学习自动打卡，并可把同日情绪记录关联到打卡。
- 情绪记录、模板反馈、打卡详情情绪展开。
- 统计页周/月/年聚合、学习趋势、情绪趋势、主情绪摘要、打卡日历和环比增长。

当前剩余的主要后端增强点是：敏感词过滤、真实 AI 反馈集成、成员加入/离开历史、番茄钟专门状态机，以及是否强制“结束学习后才能提交情绪”的产品规则。

## 2026-06-10 个人中心补充核查

本节为新增状态记录，只追加当前个人中心实现现况。

### 已补齐后端逻辑

| 功能 | API | 后端实现 | 数据库字段 |
| --- | --- | --- | --- |
| 当前用户资料扩展 | `GET /api/v1/users/me` | 已返回 `profile`、`streakDays`，个人中心资料卡可直接使用 | `users.profile`、`users.streak_days`、`users.avatar_url` |
| 修改个人资料 | `PATCH /api/v1/users/me` | 已实现昵称和个人简介修改；手机号不可修改；昵称不能为空且不超过 20 字符，简介不超过 255 字符 | `users.nickname`、`users.profile`、`users.updated_at` |
| 上传头像 | `POST /api/v1/users/me/avatar` | 已实现 multipart `file` 上传，限制 JPG/PNG 和最大 3MB；保存文件后更新用户头像地址 | `users.avatar_url` |
| 静态头像访问 | `GET /storage/avatars/{filename}` | 已在应用层挂载头像目录，便于本地 HTML 页面直接显示上传后的头像 | `AVATAR_DIR` 配置 |
| 修改密码 | `PATCH /api/v1/users/me/password` | 已校验当前密码，校验新密码不少于 8 位且包含字母和数字，更新 bcrypt 哈希 | `users.password_hash`、`users.updated_at` |

### 已接入前端联调

| 页面 | 当前状态 |
| --- | --- |
| `frontend/rooms-demo.html` | “个人中心”导航已从占位弹窗改为真实页面，支持资料展示、资料保存、头像上传、密码修改 |
| 个人中心资料卡 | 展示头像、昵称、用户 ID、今日学习、连续打卡、今日心情 |
| 个人资料表单 | 对接 `PATCH /api/v1/users/me` |
| 头像上传 | 对接 `POST /api/v1/users/me/avatar`，本地 HTML 自动把 `/storage/avatars/...` 拼接为 `http://127.0.0.1:8080/storage/avatars/...` |
| 修改密码 | 对接 `PATCH /api/v1/users/me/password` |

### 主要源码

| 文件 | 说明 |
| --- | --- |
| `backend/src/modules/users/routes.rs` | 新增个人资料、头像、密码路由 |
| `backend/src/modules/users/handler.rs` | 解析 JSON/multipart 请求并返回统一响应 |
| `backend/src/modules/users/service.rs` | 资料校验、头像保存编排、密码校验与哈希更新 |
| `backend/src/modules/users/repository.rs` | 更新用户资料、头像地址、密码哈希 |
| `backend/src/modules/users/dto.rs` | 新增资料修改、密码修改、头像上传响应 DTO |
| `backend/src/storage.rs` | 头像文件校验与保存 |
| `backend/src/app.rs` | 挂载 `/storage/avatars` 静态访问和上传体积限制 |
| `frontend/rooms-demo.html` | 个人中心联调页面和接口调用 |
| `backend/tests/auth_api_test.rs` | 覆盖资料修改和密码修改基本链路 |

### 当前仍可增强

| 增强点 | 说明 |
| --- | --- |
| 头像旧文件清理 | 当前上传新头像后不会自动删除旧头像文件 |
| 更完整的图片校验 | 当前校验 MIME 和基础文件头，未做图片解码级校验 |
| 前端正式工程化页面 | 当前仍是 HTML 联调页，后续如果引入 Vue/React，应把个人中心拆成正式组件 |

## 2026-06-10 连续打卡口径补充

连续打卡天数已统一为以下产品口径：

- 今天已打卡时，`streakDays` 返回截至今天的连续打卡天数。
- 今天未打卡时，`streakDays` 返回截至昨天的连续打卡天数。
- 如果昨天也未打卡，`streakDays` 返回 0，不继续展示更早日期的历史连续记录。

该口径同时适用于：

| 使用位置 | 数据来源 |
| --- | --- |
| 首页今日概览 | `GET /api/v1/users/me/stats/today` 实时扫描 `checkin_records` |
| 个人中心资料卡 | `GET /api/v1/users/me/stats/today` |
| 当前用户信息缓存字段 | `GET /api/v1/users/me` 返回的 `users.streak_days` |
| 补卡/有效学习结束后的缓存刷新 | `checkins::repository::recalculate_streak_days` |
