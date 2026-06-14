# RESTful API 文档

## 通用约定

| 项 | 说明 |
| --- | --- |
| 基础路径 | /api/v1 |
| 数据格式 | application/json |
| 文件上传 | multipart/form-data |
| 认证方式 | Authorization: Bearer <token> |
| 普通用户 Token | 24 小时有效 |
| 管理员 Token | 建议独立签发，claims 中 role=admin |

统一响应结构：

```json
{
  "code": 0,
  "message": "ok",
  "data": {}
}
```

错误响应结构：

```json
{
  "code": 409,
  "message": "座位已被占用",
  "data": null
}
```

## 通用错误码

| HTTP/业务码 | 含义 |
| --- | --- |
| 400 | 参数错误 |
| 401 | 未登录或 Token 无效 |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 409 | 资源冲突，例如重复占座、重复打卡 |
| 413 | 文件过大 |
| 422 | 业务校验失败 |
| 423 | 账号锁定或禁用 |
| 500 | 服务器内部错误 |

## 接口目录

| 序号 | 接口 | 方法 | 路径 | 权限 |
| --- | --- | --- | --- | --- |
| 1 | 用户注册 | POST | /auth/register | 公开 |
| 2 | 用户登录 | POST | /auth/login | 公开 |
| 3 | 当前用户信息 | GET | /users/me | 用户 |
| 4 | 修改当前用户资料 | PATCH | /users/me | 用户 |
| 5 | 上传头像 | POST | /users/me/avatar | 用户 |
| 6 | 修改密码 | PATCH | /users/me/password | 用户 |
| 7 | 今日学习概览 | GET | /users/me/stats/today | 用户 |
| 8 | 自习室列表 | GET | /rooms | 用户 |
| 9 | 创建自习室 | POST | /rooms | 用户 |
| 10 | 自习室详情 | GET | /rooms/{roomId} | 用户 |
| 11 | 开始学习 | POST | /study-sessions | 用户 |
| 12 | 当前进行中学习会话 | GET | /study-sessions/active | 用户 |
| 13 | 更新学习会话 | PATCH | /study-sessions/{sessionId} | 用户 |
| 14 | 创建休息记录 | POST | /study-sessions/{sessionId}/breaks | 用户 |
| 15 | 延长休息 | PATCH | /study-breaks/{breakId} | 用户 |
| 16 | 学习心跳 | POST | /study-sessions/{sessionId}/heartbeats | 用户 |
| 17 | 打卡日历 | GET | /checkins | 用户 |
| 18 | 某日打卡详情 | GET | /checkins/{date} | 用户 |
| 19 | 补卡 | POST | /checkins | 用户 |
| 20 | 提交情绪打卡 | POST | /study-sessions/{sessionId}/emotion-records | 用户 |
| 21 | 情绪趋势 | GET | /users/me/emotion-trends | 用户 |
| 22 | 学习统计 | GET | /users/me/stats | 用户 |
| 23 | 管理员登录 | POST | /admin/auth/login | 公开 |
| 24 | 管理员用户列表 | GET | /admin/users | 管理员 |
| 25 | 管理员修改用户状态 | PATCH | /admin/users/{userId}/status | 管理员 |
| 26 | 管理员自习室列表 | GET | /admin/rooms | 管理员 |
| 27 | 管理员修改自习室状态 | PATCH | /admin/rooms/{roomId}/status | 管理员 |
| 28 | 管理员后台概览 | GET | /admin/dashboard | 管理员 |
| 29 | 管理员操作日志 | GET | /admin/audit-logs | 管理员 |

完整 Word 转换版见 [original/AI情绪治愈自习打卡空间_RESTful_API文档_5.27.md](original/AI情绪治愈自习打卡空间_RESTful_API文档_5.27.md)。

## DTO 命名建议

Rust 后端建议按接口分离 Request/Response：

| 类型 | Rust DTO 名称 |
| --- | --- |
| 注册请求 | RegisterRequest |
| 登录请求 | LoginRequest |
| 用户响应 | UserResponse |
| 创建房间请求 | CreateRoomRequest |
| 房间响应 | RoomResponse |
| 座位响应 | SeatResponse |
| 开始学习请求 | StartStudySessionRequest |
| 学习会话响应 | StudySessionResponse |
| 创建休息请求 | CreateBreakRequest |
| 休息响应 | StudyBreakResponse |
| 情绪提交请求 | CreateEmotionRecordRequest |
| 情绪记录响应 | EmotionRecordResponse |
| 补卡请求 | CreateCheckinRequest |
| 打卡响应 | CheckinResponse |
| 管理员登录请求 | AdminLoginRequest |
| 修改用户状态请求 | UpdateUserStatusRequest |
| 修改房间状态请求 | UpdateRoomStatusRequest |

## 用户认证接口

### POST /auth/register

用户注册。

请求体：

```json
{
  "phone": "13800138000",
  "nickname": "小明同学",
  "password": "abc123456",
  "confirmPassword": "abc123456",
  "agreeTerms": true,
  "agreePrivacy": true
}
```

校验：

- phone 必须 11 位数字且唯一。
- nickname 非空且不超过 20 字符。
- password 不少于 8 位，包含字母和数字。
- confirmPassword 必须与 password 一致。
- agreeTerms 和 agreePrivacy 必须为 true。

返回：

```json
{
  "user": {
    "userId": "10001",
    "phone": "13800138000",
    "nickname": "小明同学",
    "avatarUrl": null,
    "role": "user",
    "status": "active",
    "createdAt": "2026-05-19T10:00:00Z"
  },
  "token": "jwt"
}
```

错误：400、409、422、500。

### POST /auth/login

用户登录。

请求体：

```json
{
  "phone": "13800138000",
  "password": "abc123456"
}
```

返回 user 和 token。账号 disabled 返回 423；密码错误返回 401；连续 5 次错误后锁定 30 分钟。

## 当前用户接口

### GET /users/me

返回当前登录用户信息。需用户 Token。

返回：

```json
{
  "userId": "10001",
  "phone": "13800138000",
  "nickname": "小明同学",
  "avatarUrl": "/storage/avatars/avatar.png",
  "profile": "考研党一枚，目标是上岸理想的大学！",
  "role": "user",
  "status": "active",
  "streakDays": 7,
  "createdAt": "2026-05-19T10:00:00Z"
}
```

### PATCH /users/me

修改当前用户资料。

请求体：

```json
{
  "nickname": "新的昵称",
  "profile": "个人简介"
}
```

手机号不可修改。

返回更新后的当前用户信息，字段同 `GET /users/me`。

### POST /users/me/avatar

上传头像。multipart 字段建议为 file。

限制：

- JPG 或 PNG。
- 不超过 3MB。

返回：

```json
{
  "avatarUrl": "/storage/avatars/uuid.png",
  "user": {
    "userId": "10001",
    "phone": "13800138000",
    "nickname": "小明同学",
    "avatarUrl": "/storage/avatars/uuid.png",
    "profile": "个人简介",
    "role": "user",
    "status": "active",
    "streakDays": 7,
    "createdAt": "2026-05-19T10:00:00Z"
  }
}
```

### PATCH /users/me/password

修改密码。

请求体：

```json
{
  "currentPassword": "old123456",
  "newPassword": "new123456",
  "confirmPassword": "new123456"
}
```

必须先校验 currentPassword。

成功返回 `data: null`。

### GET /users/me/stats/today

今日学习概览。

`streakDays` 口径：如果今天已打卡，返回截至今天的连续打卡天数；如果今天尚未打卡，返回截至昨天的连续打卡天数；如果昨天也未打卡，则返回 0。

返回建议：

```json
{
  "todayMinutes": 120,
  "streakDays": 5,
  "todayCheckin": true,
  "latestEmotion": "平静"
}
```

## 自习室接口

### GET /rooms

查询自习室列表。

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| keyword | string | 房间名称关键词 |
| status | string | open 或 closed |
| page | number | 默认 1 |
| pageSize | number | 默认 10 |

返回 rooms、total、page、pageSize。每个 room 应包含 currentMembers、capacity、creator、status、closeAt。

### POST /rooms

创建自习室。

请求体：

```json
{
  "name": "晨读自习室",
  "description": "一起早起学习",
  "capacity": 30,
  "isPrivate": false,
  "password": null,
  "closeAt": "2026-05-20T22:30:00Z"
}
```

校验：

- name 唯一。
- capacity 在 1-50。
- closeAt 至少晚于当前时间 1 小时。
- 创建者自动成为 creator_id。
- 建议创建默认座位，如 A01-A30。

### GET /rooms/{roomId}

获取房间详情。

返回建议：

```json
{
  "room": {},
  "seats": [],
  "members": []
}
```

seats 包含 seatId、seatCode、status、occupiedBy。

## 学习会话接口

### POST /study-sessions

开始学习。

请求体：

```json
{
  "roomId": "1",
  "seatId": "3",
  "mode": "pomodoro",
  "studyContent": "复习数据结构"
}
```

事务内校验：

- 用户无进行中会话。
- 房间存在且 open。
- 房间未满。
- 座位存在、属于该房间且 available。
- 创建 study_sessions。
- 更新 study_room_seats.status=occupied。

错误：409 座位已占用、409 房间已满、409 已存在进行中会话、422 房间已关闭。

### GET /study-sessions/active

获取当前进行中会话。无会话时可返回 null。

### PATCH /study-sessions/{sessionId}

更新学习会话状态，主要用于暂停、恢复、结束。

请求体建议：

```json
{
  "status": "ended",
  "studyContent": "复习完成"
}
```

结束学习时：

- 校验会话归属。
- 计算 duration_minutes。
- duration_minutes < 10 时 is_valid=false。
- 设置 end_time 和 status=ended。
- 释放座位。
- 必要时 upsert 当日 checkin_records。

### POST /study-sessions/{sessionId}/breaks

创建休息记录。

请求体：

```json
{
  "durationMinutes": 5
}
```

要求 session.status=studying。

### PATCH /study-breaks/{breakId}

延长休息。

请求体：

```json
{
  "extendMinutes": 5
}
```

休息已结束时返回 409。

### POST /study-sessions/{sessionId}/heartbeats

学习心跳。

请求体：

```json
{
  "clientTime": "2026-05-19T10:00:00Z"
}
```

更新 last_heartbeat_at。会话已结束返回 409。

## 打卡接口

### GET /checkins

获取打卡日历。

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| month | string | YYYY-MM |

返回当月每日打卡状态、学习分钟数、是否补卡。

### GET /checkins/{date}

获取某日打卡详情。date 格式 YYYY-MM-DD。

### POST /checkins

补卡。

请求体：

```json
{
  "date": "2026-05-18",
  "totalMinutes": 60,
  "makeupReason": "忘记提交",
  "summaryNote": "补充昨天学习记录"
}
```

校验：

- 只能补 7 日内。
- 当日不存在打卡记录。
- totalMinutes > 0。
- 文本字段需过滤敏感词。

## AI 情绪接口

### POST /study-sessions/{sessionId}/emotion-records

提交情绪打卡。

请求体：

```json
{
  "emotionTag": "焦虑",
  "emotionScore": 8,
  "userNote": "今天复习压力比较大"
}
```

校验：

- 会话存在且属于当前用户。
- emotionScore 在 1-10。
- 同一会话不可重复提交，除非产品允许多次记录。
- userNote 需过滤敏感词。

返回 emotionRecord 和 aiFeedback。

### GET /users/me/emotion-trends

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| period | string | week、month、year |

返回情绪评分趋势和情绪标签分布。

### GET /users/me/stats

学习统计。

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| period | string | week、month、year |

返回学习时长趋势、有效会话数、打卡数、平均学习时长等。

## 管理员接口

### POST /admin/auth/login

管理员登录。

请求体：

```json
{
  "account": "admin",
  "password": "admin123"
}
```

返回 admin、adminToken。

### GET /admin/users

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| keyword | string | 手机号或昵称关键词 |
| status | string | active 或 disabled |
| page | number | 默认 1 |
| pageSize | number | 默认 10 |

### PATCH /admin/users/{userId}/status

修改用户状态。

请求体：

```json
{
  "status": "disabled",
  "reason": "违规使用"
}
```

写入 audit_logs，action 建议为 user_disable 或 user_enable。

### GET /admin/rooms

后台自习室列表。支持 keyword、status、page、pageSize。

### PATCH /admin/rooms/{roomId}/status

修改自习室状态。

请求体：

```json
{
  "status": "closed",
  "reason": "维护"
}
```

写入 audit_logs，action 建议为 room_close 或 room_open。

### GET /admin/dashboard

后台概览。返回用户总数、活跃用户数、房间数、今日学习数据、打卡数、情绪分布。

### GET /admin/audit-logs

Query：

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| type | string | login、disable、enable、room_close、room_open 等 |
| startDate | string | YYYY-MM-DD |
| endDate | string | YYYY-MM-DD |
| page | number | 默认 1 |
| pageSize | number | 默认 10 |

