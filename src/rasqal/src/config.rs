use crate::runtime::ActiveTracers;

pub struct RasqalConfig {
  /// How many steps the symbolic executor is allowed to make before failing.
  pub step_count_limit: Option<i64>,
  pub debug_tracers: ActiveTracers
}

impl RasqalConfig {
  pub fn step_count_limit(&mut self, count: i64) { self.step_count_limit = Some(count); }

  pub fn trace_runtime(&mut self) { self.debug_tracers.insert(ActiveTracers::Runtime); }

  pub fn trace_projections(&mut self) { self.debug_tracers.insert(ActiveTracers::Projections); }

  pub fn trace_graphs(&mut self) { self.debug_tracers.insert(ActiveTracers::Graphs); }
}

impl Default for RasqalConfig {
  fn default() -> Self {
    RasqalConfig {
      step_count_limit: None,
      debug_tracers: ActiveTracers::empty()
    }
  }
}
