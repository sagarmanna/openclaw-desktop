import React from "react";

export default function ApprovalCard({ approval, onApprove, onReject }) {
  if (!approval) return null;

  const preview = typeof approval.preview_json === "string"
    ? approval.preview_json
    : JSON.stringify(approval.preview_json, null, 2);

  return (
    <div className="card">
      <div className="row">
        <span className="badge warn">Approval needed</span>
        <span className="badge">{approval.action_type}</span>
      </div>
      <div className="small" style={{ marginTop: 8 }}>
        This action is “public / risky”. Review preview below, then approve or reject.
      </div>
      <pre className="card" style={{ marginTop: 10, overflow: "auto" }}>
{preview}
      </pre>

      <div className="row" style={{ marginTop: 10, justifyContent: "flex-end" }}>
        <button className="iconBtn" onClick={onReject}>Reject</button>
        <button className="sendBtn" onClick={onApprove}>Approve</button>
      </div>
    </div>
  );
}
