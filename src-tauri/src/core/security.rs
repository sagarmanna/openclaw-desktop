
pub fn is_allowed_command(cmd: &str) -> bool {
  matches!(cmd, "openclaw" | "python" | "node")
}
