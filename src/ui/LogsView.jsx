import React, { useEffect, useState } from "react";
import { api } from "../lib/tauri.js";

export default function LogsView() {
  const [logs, setLogs] = useState([]);
  const [err, setErr] = useState("");

  async function load() {
    try {
      setErr("");
      const res = await api.listLogs(80);
      setLogs(res);
    } catch (e) {
      setErr(String(e));
    }
  }

  useEffect(() => {
    load();
  }, []);

  return (
    <div style={{ padding: 12, overflow: "auto" }}>
      <div className="row" style={{ justifyContent: "space-between", marginBottom: 10 }}>
        <div className="badge">Logs</div>
        <button className="iconBtn" onClick={load}>Refresh</button>
      </div>

      {err && <div className="card" style={{ borderColor: "rgba(255,110,110,.3)" }}>{err}</div>}

      <div style={{ display: "grid", gap: 8 }}>
        {logs.map((l) => (
          <div key={l.id} className="card">
            <div className="row" style={{ justifyContent: "space-between" }}>
              <span className={`badge ${l.level === "ERROR" ? "warn" : "ok"}`}>{l.level}</span>
              <span className="badge">{new Date(l.created_at).toLocaleString()}</span>
            </div>
            <div style={{ marginTop: 8 }}>{l.message}</div>
            {l.meta_json && (
              <pre className="small" style={{ marginTop: 8, overflow: "auto" }}>
{l.meta_json}
              </pre>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
