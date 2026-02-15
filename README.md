🚀 OpenClaw Desktop

A chat-first AI agent control panel built with Tauri (Rust) + React (Vite).

This desktop app provides:

🧠 Local-first LLM routing (local stub → external API key)

💬 Floating chat assistant UI

🗂 Agent creation + approval flow

📅 Scheduling system

🗃 SQLite storage (agents, logs, settings)

🛡 Sandbox / dry-run mode

🔒 Approval required for risky/public actions

🏗 Tech Stack

Frontend: React + Vite

Backend: Tauri (Rust)

Database: SQLite (via r2d2_sqlite)

State Management: React Context + Reducer

LLM Router: Local stub + external provider support

📁 Project Structure
openclaw-desktop/
├─ src/               # React frontend
├─ src-tauri/         # Rust backend
│  ├─ commands/       # Tauri commands
│  ├─ services/       # LLM, DB, process logic
│  ├─ models/         # Data models
│  └─ migrations/     # SQLite schema
└─ README.md

⚙️ Requirements (Windows)

Node.js 18+

Rust (via rustup)

Visual Studio Build Tools (Desktop development with C++)

Windows 10/11 SDK
