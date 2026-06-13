# ai-study-room

> 前端项目文档

## 1. 项目简介

一个在线自习室应用，提供虚拟自习室环境下的学习计时、座位选择、情绪记录与数据分析功能。

### 核心功能

- 用户注册与登录（含管理员登录）
- 自习室浏览、创建与详情查看
- 虚拟座位选择（25 座 5x5 网格布局）
- 学习计时（正向计时 + 番茄钟模式）
- 学习情绪记录与 AI 反馈
- 学习统计数据与趋势图表
- 打卡日历

---

## 2. 技术栈

| 技术 | 用途 | 版本 |
|------|------|------|
| React | UI 框架 | ^18.3.1 |
| Vite | 构建工具 | ^8.0.12 |
| Tailwind CSS | 样式框架 | ^4.3.0 |
| Axios | HTTP 客户端 | ^1.17.0 |
| Chart.js | 图表渲染 | ^4.5.1 |
| Font Awesome | 图标库（Free） | ^7.2.0 |
| Vitest | 单元测试 | ^4.1.8 |

### 关于路由

页面切换通过 `App.jsx` 中的 `currentPage` 状态配合 `switch-case` 和 `React.lazy` 实现手工路由。

---

## 3. 项目结构

```
frontend/
├── index.html                  # 入口 HTML
├── vite.config.js              # Vite 配置（含 API 代理）
├── .env                        # 环境变量（API Base URL）
├── package.json
├── tsconfig.json               # TypeScript 配置（当前未使用 TS 源码）
├── eslint.config.js
├── public/
│   ├── favicon.svg
│   └── icons.svg
├── demo.html                   # UI 原型参考
└── src/
    ├── main.jsx                # 应用入口
    ├── App.jsx                 # 根组件 + 手工路由
    ├── index.css               # Tailwind 入口 + 自定义主题
    ├── api/                    # API 请求层
    │   ├── request.js          # Axios 统一封装（拦截器、token 注入）
    │   ├── index.js            # API 统一导出入口
    │   ├── auth.js             # 认证（登录/注册）
    │   ├── user.js             # 用户信息
    │   ├── room.js             # 自习室
    │   ├── study.js            # 学习会话
    │   ├── emotion.js          # 情绪记录
    │   ├── statistics.js       # 学习统计
    │   ├── checkin.js          # 打卡
    │   └── admin.js            # 管理后台
    ├── components/             # UI 组件
    │   ├── common/
    │   │   └── ModalWrapper.jsx
    │   ├── emotion/
    │   │   ├── EmotionCard.jsx
    │   │   ├── EmotionTrend.jsx
    │   │   └── AiFeedbackCard.jsx
    │   ├── feedback/
    │   │   ├── DisabledAccountModal.jsx
    │   │   ├── DuplicateSessionModal.jsx
    │   │   ├── PrivacyModal.jsx
    │   │   ├── SensitiveWordModal.jsx
    │   │   └── TermsModal.jsx
    │   ├── Navbar.jsx
    │   └── CreateRoomModal.jsx
    ├── pages/                  # 页面组件
    │   ├── login/index.jsx     # 登录/注册/管理员登录
    │   ├── home/index.jsx      # 首页
    │   ├── room/index.jsx      # 自习室大厅（别名 HallList）
    │   ├── study/
    │   │   ├── HallList.jsx    # 自习室大厅（代理到 room/index）
    │   │   ├── RoomDetail.jsx  # 自习室详情
    │   │   └── StudyTimer.jsx  # 学习计时
    │   ├── dashboard/index.jsx # 学习统计
    │   ├── emotion/index.jsx   # 情绪分析
    │   ├── profile/index.jsx   # 个人中心
    │   └── admin/index.jsx     # 管理后台
    ├── store/                  # 全局状态管理
    │   ├── userContext.jsx     # 用户信息（仅 GET /users/me）
    │   └── studyContext.jsx    # 学习会话生命周期
    ├── viewmodels/             # 数据适配层（DTO → UI Model）
    │   ├── index.js
    │   ├── _userVM.js          # 用户 VM 纯函数
    │   ├── userVM.js           # 用户 VM（React Hook 包装）
    │   ├── roomVM.js           # 自习室 VM
    │   ├── sessionVM.js        # 学习会话 VM
    │   └── emotionVM.js        # 情绪 VM
    ├── hooks/
    │   └── useTimer.js         # 计时器 Hook
    ├── constants/
    │   ├── emotions.js         # 情绪常量定义
    │   └── studySessionStatus.js
    ├── utils/
    │   └── index.js            # 工具函数
    └── __tests__/              # 测试
        ├── setup.js
        ├── utils.test.js
        └── components.test.jsx
```

### 目录职责

| 目录 | 职责 |
|------|------|
| `api/` | 纯请求层，封装 Axios 调用，返回原始 DTO，不处理业务逻辑 |
| `viewmodels/` | DTO → UI Model 转换层，隔离后端数据结构与 UI |
| `store/` | React Context 全局状态管理 |
| `components/` | 可复用 UI 组件 |
| `pages/` | 页面级组件，负责组合 UI 组件与数据获取 |
| `hooks/` | 自定义 React Hooks |
| `constants/` | 全局常量定义 |
| `utils/` | 纯工具函数 |

---

## 4. 页面与功能模块

### 登录 / 注册（`/pages/login/index.jsx`）

- 三 tab 切换：用户登录、用户注册、管理员登录
- 数据来源：`authApi.login()` / `authApi.register()` / `adminApi.login()`
- 注册字段：手机号、昵称、密码、确认密码、用户协议同意
- 内置敏感词检测（注册昵称）
- 模态框组件：DisabledAccountModal、TermsModal、PrivacyModal、SensitiveWordModal

### 首页（`/pages/home/index.jsx`）

- 欢迎区域：用户昵称、今日学习时长、连续打卡天数、今日心情
- 快速入口："进入自习室" 和 "创建自习室" 按钮
- 推荐自习室列表（取前 3 条开放房间）
- 数据来源：`userVM`（UserContext）、`roomApi.getRoomList()`、`statisticsApi.getTodayStats()`

### 自习室大厅（`/pages/room/index.jsx`）

- 自习室网格列表（含渐变色彩色卡片）
- 搜索过滤功能
- 创建自习室模态框
- 学习进行中时重复进入拦截（DuplicateSessionModal）
- 数据来源：`roomApi.getRoomList()`

### 自习室详情（`/pages/study/RoomDetail.jsx`）

- 房间信息展示（名称、描述、容量、开放时长）
- 座位网格（5x5 布局，三色区分可用/已占用/已选择）
- 已占用座位显示占用用户头像
- 成员列表（含房主标签）
- 选座交互
- "锁定座位并开始学习" 按钮
- 数据来源：`roomApi.getRoomDetail()`

### 学习计时（`/pages/study/StudyTimer.jsx`）

- 正向计时 + 番茄钟双模式
- 计时器控制：开始、暂停、恢复、结束
- 学习结束时间不足 10 分钟拦截确认
- 番茄钟：25 分钟学习 + 5 分钟休息
- 番茄钟圆形进度图
- 学习结束后的情绪记录弹窗
- AI 反馈展示弹窗
- 数据来源：`StudyContext`（全局状态）

### 情绪分析（`/pages/emotion/index.jsx`）

- 当前情绪状态卡片
- 情绪趋势展示（按周/月/年切换）
- 主导情绪与总结
- 近期情绪记录列表
- AI 反馈展示区域
- 数据来源：`emotionApi.getEmotionTrend()`

### 学习统计（`/pages/dashboard/index.jsx`）

- 四个统计卡片：总学习时长、日均学习、连续打卡、完成打卡
- 学习时长折线图（Chart.js）
- 情绪变化折线图（Chart.js）
- 打卡日历（含今日/已打卡/未打卡标识）
- 周期切换（周/月/年）
- 数据来源：`statisticsApi.getStudyStats()`、`emotionApi.getEmotionTrend()`、`checkinApi.getCalendar()`

### 个人中心（`/pages/profile/index.jsx`）

- 用户头像、昵称、手机号、注册时间展示
- "查看学习统计" 跳转按钮
- "编辑个人资料" 和 "修改密码" 按钮（已禁用，标注"即将上线"）
- 退出登录
- 数据来源：`UserContext`

### 管理后台（`/pages/admin/index.jsx`）

- 侧边导航：仪表盘、用户管理、房间管理、审计日志
- 所有四个 tab 展示占位符
- 退出管理功能
- 数据来源：仅管理员登录 API 可用，其余后端未实现

---

## 5. 状态管理架构

项目使用 React Context 进行全局状态管理，共两个 Context。

### UserContext（`/store/userContext.jsx`）

| 属性 | 说明 |
|------|------|
| 管理内容 | 当前登录用户完整信息 |
| 数据来源 | 挂载时调用 `userApi.getProfile()`（GET /users/me） |
| 更新时机 | 登录成功 → `setUser()`；退出 → `clearUser()`；手动刷新 → `refreshUser()` |
| 生命周期 | 应用挂载时自动请求（仅一次），token 不存在时跳过 |

**数据流向**：UserContext → `useUserVM()` Hook → 各页面组件

### StudyContext（`/store/studyContext.jsx`）

| 属性 | 说明 |
|------|------|
| 管理内容 | 学习会话状态（sessionId、status、startedAt、duration、timerMode） |
| 数据来源 | 后端 study-sessions API + localStorage 持久化 |
| 更新时机 | startStudy / pauseStudy / resumeStudy / endStudy / submitEmotion |
| 刷新恢复 | 页面刷新时优先请求 `studyApi.getActiveSession()`，后端不可达时 fallback 到 localStorage |

**数据流向**：StudyContext → StudyTimer 页面

---

## 6. 数据流架构

项目采用 **API → ViewModel → UI** 三层数据流架构。

```
后端 API (DTO)
    ↓
api/ 层（Axios 请求 + 响应解包）
    ↓
viewmodels/ 层（DTO → UI Model 转换）
    ↓
页面/组件层（仅使用 ViewModel 字段）
```

### api 层（`/api/`）

- 职责：发送 HTTP 请求、接收后端响应、解包统一响应结构 `{ code, message, data }`
- 约束：只做请求和响应传输，不处理业务逻辑
- 统一封装：`request.js`（Axios 实例 + token 注入 + 401 拦截）

### ViewModel 层（`/viewmodels/`）

- 职责：将后端 DTO 字段映射为 UI 可用的 ViewModel
- 示例映射：
  - `seat.status === "occupied"` → `seat.isOccupied = true`
  - `room.creator.nickname` → `room.creatorName`
  - `emotionTag + EMOTION_SCORE_MAP` → `emotionScore`
- 约束：UI 组件不得直接访问后端字段名

### 页面层

- 职责：调用 API → 通过 ViewModel 转换 → 渲染 UI
- 约束：不直接操作 API 响应原始数据

---

## 7. API 集成情况和遗留问题

### 已集成（前端已调用，后端已实现）

| 模块 | API 文件 |
|------|----------|
| 登录 | `authApi.login()` |
| 注册 | `authApi.register()` |
| 管理员登录 | `adminApi.login()` |
| 获取用户信息 | `userApi.getProfile()` |
| 自习室列表 | `roomApi.getRoomList()` |
| 自习室详情 | `roomApi.getRoomDetail()` |
| 创建自习室 | `roomApi.createRoom()` |
| 开始/暂停/恢复/结束学习 | `studyApi.*` |
| 学习心跳 | `studyApi.sendHeartbeat()` |
| 创建/延长休息 | `studyApi.createBreak()` / `studyApi.extendBreak()` |
| 提交情绪记录 | `emotionApi.submitEmotion()` |
| 获取情绪趋势 | `emotionApi.getEmotionTrend()` |
| 今日学习概览 | `statisticsApi.getTodayStats()` |
| 学习统计 | `statisticsApi.getStudyStats()` |
| 打卡日历 | `checkinApi.getCalendar()` |

### 已定义但后端未实现（前端已禁用）

- AI 反馈提交（POST /users/me/emotion-records/{id}/feedback）
- 管理后台仪表盘（GET /admin/dashboard）
- 用户管理（GET /admin/users, PATCH /admin/users/{id}/status）
- 房间管理（GET /admin/rooms, PATCH /admin/rooms/{id}/status）
- 审计日志（GET /admin/audit-logs）


### 后端未实现部分与前端冲突性检查

| 后端缺口 | 前端行为 | 状态 |
|------|----------|------|
| 首页推荐自习室/最近房间专用接口 | pages/home/index.jsx 调用 roomApi.getRoomList({ status: "open", page: 1, pageSize: 3 })，即通用 GET /rooms 接口 + 参数筛选，未调用任何专用推荐接口。 | ✅ 无冲突
| 私密房间密码校验 | CreateRoomModal.jsx 硬编码 isPrivate: false，room.js 无密码相关 API，无任何密码校验调用。 | ✅ 无冲突
| 正向计时/番茄钟模式完整计时器逻辑 | 全部计时逻辑（番茄钟轮次、专注/休息切换、模式互换）由前端 hooks/useTimer.js 纯客户端实现，仅 mode 和 status 提交到后端。 | ✅ 无冲突
| 房间成员历史/在线状态更细粒度	| 前端仅展示 GET /rooms/{roomId} 返回的当前 members 数组，未调用任何历史/在线状态 API。 | ✅ 无冲突
| 情绪入口未强制 session 必须 ended | 在 StudyTimer.jsx 中流程为：endStudy() (结束会话) → setShowEmotion(true) (弹出情绪窗口) → submitEmotion() (提交情绪)。情绪提交在 endStudy 之后，sessionId 通过 lastSessionIdRef.current 传递。 | ✅ 无冲突

- 以上 5 个"后端缺口"在当前前端代码中均不存在对应的调用或依赖，前端实现在这些方面与后端现状兼容，没有因后端缺失而产生的功能断裂。

### 前后端整合产生的问题
- 统计数据未反映新学习/情绪记录
  前端链路验证：创建会话 → 学习 → 心跳 → 结束 → 提交情绪 → 跳转 → 获取统计，全链路完整且每次获取统计均为组件挂载时实时发起的新请求。
  可能原因：后端统计接口的实现逻辑，例如：
  - 后端统计接口查询时未包含刚结束的会话（时间窗口截断、事务未提交、或有缓存）
  - 后端 PATCH /study-sessions/{id} handler 未正确设置 end_time / duration_minutes / is_valid 等字段
  - 后端 GET /users/me/stats 使用预计算而非实时聚合
- 座位号编码总是 A 开头
  可能原因：
  backend/src/modules/rooms/seat_code.rs：pub fn seat_code(index: i32) -> String { format!("A{:02}", index) }
  该函数固定使用 A 前缀，没有将 room_id 纳入计算。seed 数据中 Room 2 的座位 B01/B02 是手动硬编码的，运行时通过 createRoom 创建的座位全部为 A01/A02/A03...

---

## 8. 本地开发

### 环境要求

- Node.js >= 18
- npm

### 安装依赖

```bash
npm install
```

### 启动开发服务器

```bash
npm run dev
```

开发模式下 Vite 代理 `/api` 请求到 `http://127.0.0.1:8080`（配置见 `vite.config.js`）。

### 构建生产版本

```bash
npm run build
```

### 预览构建结果

```bash
npm run preview
```

### 运行测试

```bash
npm run test:watch
```

测试框架：Vitest + jsdom + @testing-library/react
测试覆盖范围：ModalWrapper 组件渲染、工具函数（敏感词检测、日期格式化）

---

## 9. 环境配置

### 环境变量

项目在根目录包含 `.env` 文件：

```
VITE_API_BASE_URL=/api/v1
```

`request.js` 中使用此变量作为 Axios baseURL：

```javascript
baseURL: import.meta.env.VITE_API_BASE_URL || "/api/v1"
```

### 开发代理

`vite.config.js` 配置了 API 代理，将 `/api` 前缀的请求转发到后端：

```
/api → http://127.0.0.1:8080
```

后端需在 8080 端口启动。

---

## 10. 当前系统状态

### 已完成

- 用户注册与登录（含 token 持久化与自动恢复）
- 管理员登录
- 用户信息展示（Navbar、个人中心）
- 个人中心：编辑资料 / 修改密码 / 上传头像已接入
- 自习室列表、创建、详情（含座位网格与成员列表）
- 学习计时完整生命周期（正向计时 + 番茄钟 + 休息管理）
- 学习心跳（30 秒间隔自动发送）
- 页面刷新时学习会话恢复（API 优先 + localStorage 回退）
- 情绪提交与趋势查询
- 学习统计与打卡日历
- 首页今日概览
- AI 反馈弹窗：修复后端 aiFeedback 为 null 时弹窗显示空白的问题，始终调用 toAiFeedbackVM 注入 EMOTION_COMFORT_MAP 默认安慰文本

### 部分完成
- 学习统计：图表渲染已完成，环比变化数据未接入

### 未完成

- 管理后台全部页面（4 个 tab 展示占位符，后端 6 个 API 未实现）
- 加入/离开房间功能（后端未提供接口，已从前端移除）
- 补卡功能（POST /checkins 已定义但页面无入口）

### 已知限制

- 学习结束时情绪提交依赖 sessionId，存在时序风险（已修复）
- 图片占位使用 picsum.photos 外部服务，不可用时可能出现破损图片
- 后端统一响应格式 `{ code, message, data }` 为约定式，无 schema 校验

---

## 11. 项目文档

| 文件 | 说明 |
|------|------|
| `status_report.md` | 系统运行状态，含前端状态、API 联调状态、数据流分析 |
| `demo.html` | UI 原型参考文件，内含所有页面完整视觉设计 |