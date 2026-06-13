# SYSTEM_STATUS.md


## 前端状态

### 技术栈
- React 19 + Vite + Tailwind CSS v4
- 路由：App.jsx 手工路由（switch-case + Suspense lazy loading）
- 状态管理：React Context（UserContext + StudyContext）
- HTTP 客户端：Axios（request.js 统一封装）
- 图表：Chart.js
- 构建：Vite

### 页面路由表
| 页面 | 路径标识 | 状态 |
|------|----------|------|
| Login | `currentPage="login"` | 已完成，API 已接入 |
| Home | `currentPage="home"` | 已完成，API 已接入 |
| 自习室大厅 | `currentPage="study-rooms"` | 已完成，API 已接入 |
| 自习室详情 | `currentPage="room-detail"` | 已完成，API 已接入 |
| 学习计时 | `currentPage="study-timer"` | 已完成，API 已接入 |
| 学习统计 | `currentPage="statistics"` | 已完成，API 已接入 |
| 情绪分析 | `currentPage="emotion"` | 已完成，API 已接入 |
| 个人中心 | `currentPage="profile"` | 部分完成，编辑功能禁用 |
| 管理后台 | `currentPage="admin"` | 仅登录可用，其余占位符 |

### Navbar
- 固定顶部 + glass effect 毛玻璃效果
- 导航项：首页、自习室、学习统计、个人中心
- 右上角：用户头像 + 昵称 + 下拉菜单（含退出登录）
- 移动端支持汉堡菜单

---

## 后端依赖状态（仅根据前端调用判断）

### 已确认实现的 Backend API
| 接口 | 来源判断 | 前端使用 |
|------|----------|----------|
| POST /auth/register | auth.js 直接调用 | LoginPage handleRegister |
| POST /auth/login | auth.js 直接调用 | LoginPage handleLogin |
| GET /users/me | userContext.jsx 挂载时调用 | 全局用户信息 |
| GET /rooms | room.js api + page 调用 | HallList / HomePage 列表渲染 |
| POST /rooms | room.js api | CreateRoomModal |
| GET /rooms/{roomId} | room.js api + page 调用 | RoomDetail 详情 |
| POST /study-sessions | study.js api | StudyContext startStudy |
| GET /study-sessions/active | study.js api | StudyContext 刷新恢复 |
| PATCH /study-sessions/{id} | study.js api | 暂停/恢复/结束 |
| POST /study-sessions/{id}/breaks | study.js api | 休息记录 |
| PATCH /study-breaks/{id} | study.js api | 延长休息 |
| POST /study-sessions/{id}/heartbeats | study.js api | 心跳（30s间隔） |
| POST /study-sessions/{id}/emotion-records | emotion.js api | 情绪提交 |
| GET /users/me/emotion-trends | emotion.js api | 情绪趋势/列表 |
| GET /users/me/stats/today | statistics.js api | 首页今日概览 |
| GET /users/me/stats | statistics.js api | 学习统计页面 |
| GET /checkins | checkin.js api | 打卡日历 |
| GET /checkins/{date} | checkin.js api | 定义但未确认使用 |
| POST /checkins | checkin.js api | 定义但无UI入口 |
| POST /admin/auth/login | admin.js api | 管理后台登录 |

### 已确认缺失的 Backend API
| 接口 | 前端表现 | 证据 |
|------|----------|------|
| PATCH /users/me | 按钮禁用 + "即将上线" | user.js 注释标注 |
| POST /users/me/avatar | 按钮禁用 + "即将上线" | user.js 注释标注 |
| PATCH /users/me/password | 按钮禁用 + "即将上线" | user.js 注释标注 |
| POST /users/me/emotion-records/{id}/feedback | 功能禁用 | emotionVM 注释标注 |
| GET /admin/dashboard | 占位符 | admin.js 注释标注 |
| GET /admin/users | 占位符 | admin.js 注释标注 |
| PATCH /admin/users/{id}/status | 占位符 | admin.js 注释标注 |
| GET /admin/rooms | 占位符 | admin.js 注释标注 |
| PATCH /admin/rooms/{id}/status | 占位符 | admin.js 注释标注 |
| GET /admin/audit-logs | 占位符 | admin.js 注释标注 |

---

## API 联调状态

### 已成功联调
- 认证（注册 / 登录 / 管理员登录）
- 用户信息获取
- 自习室 CRUD（列表 / 创建 / 详情）
- 学习会话全生命周期（开始 / 暂停 / 恢复 / 结束 / 心跳）
- 情绪记录（提交 + 趋势查询）
- 学习统计（今日概览 + 全面统计）
- 打卡日历

### 部分联调
- 情绪趋势数据展示正常，但 AI 反馈提交接口缺失

### 未联调
- 管理后台（6个接口全部缺失）
- 用户资料更新（3个接口全部缺失）
- 加入/离开房间（无接口）

---

## 数据流状态

### 数据来源分布

| 数据类型 | 来源 | 传输方式 |
|----------|------|----------|
| 用户认证信息 | Backend (JWT) | localStorage token + UserContext |
| 用户个人信息 | Backend (GET /users/me) | UserContext 全局共享 |
| 自习室列表 | Backend (GET /rooms) | 页面级 local state |
| 自习室详情 | Backend (GET /rooms/{id}) | 页面级 local state |
| 座位数据 | Backend (GET /rooms/{id} 内含 seats) | ViewModel 转换后渲染 |
| 学习会话 | Backend (study-sessions API) | StudyContext 全局共享 |
| 情绪数据 | Backend (emotion-records + trends) | 页面级 local state |
| 统计数据 | Backend (stats API) | 页面级 local state |
| 打卡数据 | Backend (checkins API) | 页面级 local state |
| 管理后台数据 | 无 | 占位符（无API） |

### ViewModel 层隔离
- 所有 Backend DTO 经过 viewmodels/ 层转换后提供给 UI
- roomVM.js：RoomResponse → RoomVM（字段重命名、状态归一化）
- userVM.js：UserResponse → UserVM（字段归一化）
- emotionVM.js：EmotionDTO → EmotionPayload / AIfeedbackVM（字段兼容）
- sessionVM.js：SessionResponse → SessionVM（单位转换）
- UI 组件不直接访问后端字段名，通过 useUserVM / to*VM 获取数据

### Mock 数据使用情况
- 前端无任何 mock 数据残留
- 所有数据均来自后端 API 调用
- 图片资源使用 picsum.photos 占位图（头像/插图），非业务 mock

---

## 已知不稳定模块

### 1. 学习计时页面的番茄钟模式
- 风险：`pausedAt` / `resumedAt` 不被后端 DTO 反序列化，仅前端本地展示
- 状态：本地正常，但后端更新 session 时不持久化暂停/恢复时间戳

### 2. 学习心跳恢复
- 风险：页面刷新恢复时，如果后端不可达，会 fallback 到 localStorage 本地数据
- 状态：有 fallback 机制，但可能产生前后端状态不一致

### 3. 管理后台
- 风险：4个tab全部展示占位符，非功能缺失而是 API 缺失
- 状态：前端 UI 框架已就绪，等待后端 6 个接口实现后接入

### 4. 个人中心编辑功能
- 风险：编辑资料/修改密码/上传头像全部禁用
- 状态：等待后端 3 个接口实现
