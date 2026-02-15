import React from "react";

export default function MessageList({ messages }) {
  return (
    <div className="msgList">
      {messages.map((m) => (
        <div key={m.id} className={`msg ${m.role}`}>
          {m.text}
          <div className="small">
            {new Date(m.ts).toLocaleTimeString()}
          </div>
        </div>
      ))}
    </div>
  );
}
