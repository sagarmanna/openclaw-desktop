import React, { useState } from "react";

export default function Composer({ onSend, disabled }) {
  const [text, setText] = useState("");

  async function submit(e) {
    e.preventDefault();
    const t = text.trim();
    if (!t) return;
    setText("");
    await onSend(t);
  }

  return (
    <form className="composer" onSubmit={submit}>
      <input
        className="input"
        value={text}
        placeholder="Type a message…"
        onChange={(e) => setText(e.target.value)}
        disabled={disabled}
      />
      <button className="sendBtn" type="submit" disabled={disabled}>
        Send
      </button>
    </form>
  );
}
