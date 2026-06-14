# AI 情绪治愈自习打卡空间

这是一个面向自习场景的全栈应用，支持注册登录、自习室管理、学习计时、有效学习打卡、情绪记录、AI 情绪反馈、个人统计和管理后台。

## 主要功能

- 用户注册、登录、JWT 鉴权、个人资料管理
- 自习室列表、创建、详情、座位管理和头像展示
- 学习会话的开始、暂停、恢复、心跳、结束和有效性判断
- 学习结束后的情绪提交、AI 反馈和打卡关联
- 打卡日历、补卡和连续打卡统计
- 个人学习统计、学习趋势、情绪趋势和主情绪摘要
- 管理端后台：数据概览、用户管理、自习室管理和审计日志

## 技术栈

- 后端：Rust、Axum、SQLx、MySQL、Redis、JWT、bcrypt
- 前端：React、Vite、Axios、Chart.js、Tailwind CSS
- AI：DashScope 兼容接口，模型占位为 `qwen-math-turbo`

## 本地运行

### 1. 后端环境

复制环境变量文件：

```powershell
cd backend
Copy-Item .env.example .env
```

修改 `backend/.env` 中的本地配置：

- `DATABASE_URL`
- `DATABASE_ENABLED=true`
- `AI_API_KEY`
- `AI_MODEL=qwen-math-turbo`

启动后端：

```powershell
cd backend
cargo run
```

后端默认地址：

```text
http://127.0.0.1:8080/api/v1
```

### 2. 前端环境

安装依赖：

```powershell
cd frontend
npm install
```

启动前端：

```powershell
npm run dev
```

### 3. 校验命令

```powershell
cd backend
cargo fmt --all
cargo check

cd ..\frontend
npm run build
```

## 文件结构

```text
rjgc_code/
├─ backend/                Rust 后端工程
│  ├─ migrations/          数据库迁移脚本
│  ├─ scripts/             本地开发脚本
│  ├─ src/                 后端源码
│  │  ├─ ai/               AI 反馈逻辑
│  │  ├─ auth/             鉴权、JWT、密码处理
│  │  ├─ constants/        角色、状态等常量
│  │  ├─ modules/          业务模块
│  │  ├─ routes/           路由汇总
│  │  ├─ app.rs            应用装配
│  │  ├─ config.rs         配置读取
│  │  ├─ error.rs          统一错误类型
│  │  ├─ main.rs           启动入口
│  │  ├─ response.rs       统一响应结构
│  │  └─ state.rs          应用共享状态
│  ├─ tests/               接口集成测试
│  ├─ .env.example         环境变量示例
│  ├─ Cargo.toml
│  └─ rust-toolchain.toml
├─ frontend/               React 前端工程
│  ├─ public/              公共静态资源
│  ├─ src/                 前端源码
│  │  ├─ api/              接口封装
│  │  ├─ components/       公共组件
│  │  ├─ constants/        常量
│  │  ├─ hooks/            自定义 Hook
│  │  ├─ pages/            页面
│  │  ├─ store/            全局状态
│  │  ├─ utils/            工具函数
│  │  └─ viewmodels/       数据视图模型
│  ├─ index.html
│  ├─ vite.config.js
│  └─ package.json
├─ sql/                    原始数据库脚本
├─ README.md               项目说明
└─ .gitignore              忽略规则
```

## GitHub 提交说明

以下内容不会提交到 GitHub：

- `backend/.env`
- `backend/target/`
- `backend/storage/`
- `frontend/dist/`
- `_extracted_docx/`
- `*.doc`
- `*.docx`
- `*.md` 中除根目录 `README.md` 外的所有文件
- `docs/`

## 说明

- 管理员账号、开发种子数据和 AI Key 都应该只保存在本地环境中。
- 如果后续新增本地文档，也不要放进提交范围。