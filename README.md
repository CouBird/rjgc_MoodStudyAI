# AI Mood Study Room

A full-stack study-room app for focused learning, session tracking, check-ins, emotion logging, AI emotion feedback, and admin operations.

## Highlights

- User registration, login, JWT auth, and profile management
- Study rooms with seats, room detail pages, and creator avatars
- Study session lifecycle: start, pause, resume, heartbeat, end, and validity tracking
- Check-in and makeup check-in flows
- Emotion submission after study ends, with AI feedback support
- Personal statistics: daily summary, period trends, and emotion trend views
- Admin console: dashboard, user management, room management, and audit logs

## Tech Stack

- Backend: Rust, Axum, SQLx, MySQL, Redis, JWT, bcrypt
- Frontend: React, Vite, Axios, Chart.js, Tailwind CSS
- AI: DashScope-compatible API, Qwen model placeholder in backend env config

## Project Layout

```text
backend/    Rust API server
frontend/   React app
sql/        Database schema and seed scripts
README.md   Project overview and run guide
.gitignore  Local-only and generated file filters
```

## Requirements

- Rust 1.95+
- Node.js 20+
- MySQL 8+
- PowerShell on Windows

## Backend Setup

1. Copy the environment file:

```powershell
cd backend
Copy-Item .env.example .env
```

2. Fill in local values in `backend/.env`:

- `DATABASE_URL`
- `DATABASE_ENABLED=true` for real database access
- `AI_API_KEY`
- `AI_MODEL=qwen-math-turbo`

3. Run the database migration scripts if needed.

4. Start the backend:

```powershell
cd backend
cargo run
```

The API is exposed at:

```text
http://127.0.0.1:8080/api/v1
```

## Frontend Setup

1. Install dependencies:

```powershell
cd frontend
npm install
```

2. Start the Vite app:

```powershell
npm run dev
```

The frontend talks to the backend through `/api/v1` by default.

## Local Verification

```powershell
cd backend
cargo fmt --all
cargo check

cd ..\frontend
npm run build
```

## GitHub Export Rules

Keep the following out of the repository history:

- `backend/.env`
- `backend/target/`
- `backend/storage/`
- `frontend/dist/`
- `_extracted_docx/`
- `*.doc`
- `*.docx`
- `*.md` except the root `README.md`
- `docs/`

The root `README.md` is the only Markdown document intended for GitHub.

## Notes

- The admin account, development seed data, and AI key placeholder should stay local.
- Do not commit generated build output or exported requirement documents.