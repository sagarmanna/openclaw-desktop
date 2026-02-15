import { invoke } from "@tauri-apps/api/core";

export const api = {
  detectOS: () => invoke("detect_os"),
  checkDeps: () => invoke("check_deps"),
  setupOpenClaw: () => invoke("setup_openclaw"),

  chatIntent: (text) => invoke("chat_intent", { text }),

  saveSettings: (settings) => invoke("save_settings", { settings }),
  loadSettings: () => invoke("load_settings"),

  createAgent: (payload) => invoke("create_agent", { payload }),
  listAgents: () => invoke("list_agents"),

  approveAction: (approvalId, approved) =>
    invoke("approve_action", { approvalId, approved }),

  upsertSchedule: (payload) => invoke("upsert_schedule", { payload }),

  listLogs: (limit = 50) => invoke("list_logs", { limit })
};
