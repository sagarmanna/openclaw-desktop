import React, { useMemo, useState } from "react";
import MessageList from "./MessageList.jsx";
import Composer from "./Composer.jsx";
import ApprovalCard from "./ApprovalCard.jsx";
import LogsView from "./LogsView.jsx";
import SettingsModal from "./SettingsModal.jsx";
import { api } from "../lib/tauri.js";

export default function ChatPanel({ state, dispatch }) {
  const [busy, setBusy] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);

  const tab = state.tab;

  const header = useMemo(() => {
    return tab === "chat" ? "Assistant" : "Logs";
  }, [tab]);

  async function send(text) {
    dispatch({
      type: "add_msg",
      msg: { id: crypto.randomUUID(), role: "user", text, ts: Date.now() }
    });

    setBusy(true);
    try {
      const res = await api.chatIntent(text);

      // If backend says "approval_required", show card
      if (res?.approval_required) {
        dispatch({
          type: "set_approval",
          value: {
            id: res.approval_id,
            action_type: res.action_type,
            preview_json: res.preview_json
          }
        });
      }

      dispatch({
        type: "add_msg",
        msg: {
          id: crypto.randomUUID(),
          role: "assistant",
          text: res?.message || JSON.stringify(res, null, 2),
          ts: Date.now()
        }
      });

      if (res?.suggest_tab === "logs") {
        dispatch({ type: "set_tab", value: "logs" });
      }
    } catch (e) {
      dispatch({
        type: "add_msg",
        msg: {
          id: crypto.randomUUID(),
          role: "assistant",
          text: `Error: ${String(e)}`,
          ts: Date.now()
        }
      });
    } finally {
      setBusy(false);
    }
  }

  async function approve(approved) {
    if (!state.pendingApproval) return;
    setBusy(true);
    try {
      await api.approveAction(state.pendingApproval.id, approved);

      dispatch({
        type: "add_msg",
        msg: {
          id: crypto.randomUUID(),
          role: "assistant",
          text: approved ? "Approved ✅" : "Rejected ❌",
          ts: Date.now()
        }
      });

      dispatch({ type: "clear_approval" });
    } catch (e) {
      dispatch({
        type: "add_msg",
        msg: {
          id: crypto.randomUUID(),
          role: "assistant",
          text: `Approval error: ${String(e)}`,
          ts: Date.now()
        }
      });
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <div className="panelWrap">
        <div className="panel">
          <div className="panelHeader">
            <div className="title">{header}</div>
            <div className="headerBtns">
              <button className="iconBtn" onClick={() => setSettingsOpen(true)}>
                Settings
              </button>
              <button className="iconBtn" onClick={() => dispatch({ type: "set_open", value: false })}>
                ✕
              </button>
            </div>
          </div>

          <div className="tabRow">
            <button
              className={`tab ${tab === "chat" ? "active" : ""}`}
              onClick={() => dispatch({ type: "set_tab", value: "chat" })}
            >
              Chat
            </button>
            <button
              className={`tab ${tab === "logs" ? "active" : ""}`}
              onClick={() => dispatch({ type: "set_tab", value: "logs" })}
            >
              Logs
            </button>
          </div>

          {tab === "chat" ? (
            <>
              <div style={{ padding: 12 }}>
                <ApprovalCard
                  approval={state.pendingApproval}
                  onApprove={() => approve(true)}
                  onReject={() => approve(false)}
                />
              </div>

              <MessageList messages={state.messages} />
              <Composer onSend={send} disabled={busy} />
            </>
          ) : (
            <LogsView />
          )}
        </div>
      </div>

      <SettingsModal open={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </>
  );
}
