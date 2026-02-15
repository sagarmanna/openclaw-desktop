// Minimal process manager stub.
// Later: spawn OpenClaw CLI, stream output to frontend events.

use std::thread;

pub fn run_background<F>(f: F)
where
  F: FnOnce() + Send + 'static,
{
  thread::spawn(f);
}
