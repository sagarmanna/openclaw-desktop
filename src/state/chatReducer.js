export const initialState = {
  open: false,
  tab: "chat", // chat | logs
  messages: [
    {
      id: crypto.randomUUID(),
      role: "assistant",
      text:
        "Hi! I’m OpenClaw Desktop.\n\nTry:\n• “check my system”\n• “setup openclaw”\n• “create an agent that summarizes meeting notes”\n• “show logs”",
      ts: Date.now()
    }
  ],
  pendingApproval: null // { id, action_type, preview_json }
};

export function chatReducer(state, action) {
  switch (action.type) {
    case "toggle_open":
      return { ...state, open: !state.open };
    case "set_open":
      return { ...state, open: !!action.value };
    case "set_tab":
      return { ...state, tab: action.value };
    case "add_msg":
      return { ...state, messages: [...state.messages, action.msg] };
    case "set_approval":
      return { ...state, pendingApproval: action.value };
    case "clear_approval":
      return { ...state, pendingApproval: null };
    case "reset_chat":
      return { ...initialState, open: state.open, tab: state.tab };
    default:
      return state;
  }
}
