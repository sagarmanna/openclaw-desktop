import React from "react";
import FloatingButton from "./ui/FloatingButton.jsx";
import ChatPanel from "./ui/ChatPanel.jsx";
import { StoreProvider, useStore } from "./state/store.jsx";


function AppInner() {
  const { state, dispatch } = useStore();

  return (
    <div className="container">
      <h1 style={{ margin: 0 }}>OpenClaw Desktop</h1>
      <p className="hint">
        This is a baseline chat-first desktop shell (Tauri + React) with SQLite + commands wired.
        Use the chat button to test system checks, setup flow stubs, agents/approvals, logs, and settings.
      </p>

      <FloatingButton onClick={() => dispatch({ type: "toggle_open" })} />
      {state.open && <ChatPanel state={state} dispatch={dispatch} />}
    </div>
  );
}

export default function App() {
  return (
    <StoreProvider>
      <AppInner />
    </StoreProvider>
  );
}
