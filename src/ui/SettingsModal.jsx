import React, { useEffect, useState } from "react";
import { api } from "../lib/tauri.js";
import { DEFAULT_PROVIDER } from "../lib/constants.js";

export default function SettingsModal({ open, onClose }) {
  const [provider, setProvider] = useState(DEFAULT_PROVIDER);
  const [key, setKey] = useState("");
  const [sandbox, setSandbox] = useState(true);
  const [status, setStatus] = useState("");

  useEffect(() => {
    if (!open) return;
    (async () => {
      try {
        setStatus("");
        const s = await api.loadSettings();
        setProvider(s.user_llm_provider || DEFAULT_PROVIDER);
        setKey(s.user_llm_key || "");
        setSandbox(!!s.sandbox_enabled);
      } catch (e) {
        setStatus(String(e));
      }
    })();
  }, [open]);

  async function save() {
    try {
      setStatus("Saving…");
      await api.saveSettings({
        user_llm_provider: provider,
        user_llm_key: key ? key : null,
        sandbox_enabled: sandbox
      });
      setStatus("Saved ✅");
      setTimeout(() => onClose(), 500);
    } catch (e) {
      setStatus(String(e));
    }
  }

  if (!open) return null;

  return (
    <div className="modalOverlay" onMouseDown={onClose}>
      <div className="modal" onMouseDown={(e) => e.stopPropagation()}>
        <h3>Settings</h3>

        <div className="field">
          <label>LLM Provider</label>
          <select value={provider} onChange={(e) => setProvider(e.target.value)}>
            <option value="openai">openai</option>
            <option value="anthropic">anthropic</option>
            <option value="other">other</option>
          </select>
        </div>

        <div className="field">
          <label>API Key (optional — if empty, uses local model stub)</label>
          <input
            value={key}
            onChange={(e) => setKey(e.target.value)}
            placeholder="paste your key"
          />
        </div>

        <div className="field">
          <label>
            <input
              type="checkbox"
              checked={sandbox}
              onChange={(e) => setSandbox(e.target.checked)}
            />{" "}
            Sandbox (dry-run)
          </label>
        </div>

        {status && <div className="small">{status}</div>}

        <div className="modalFooter">
          <button className="iconBtn" onClick={onClose}>Cancel</button>
          <button className="sendBtn" onClick={save}>Save</button>
        </div>
      </div>
    </div>
  );
}
