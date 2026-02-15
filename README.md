# OpenClaw Desktop (Baseline)

## Run (web only)
npm install
npm run dev

## Run (Tauri desktop)
npm install
npm run tauri:dev

This baseline includes:
- React chat UI (floating button + chat panel)
- Tauri backend commands
- SQLite (r2d2_sqlite) + migrations
- Agents, approvals, schedules, logs (basic)
- LLM router stub: local vs external based on saved API key
