use crate::runtime::ActiveTracers;

pub struct RasqalConfig {
  /// How many steps the symbolic executor is allowed to make before failing.
  pub step_count_limit: Option<i64>,

  /// Currently active debug tracers.
  pub debug_tracers: ActiveTracers,

  /// Whether projection circuit solving should be activated. If this is true every circuit will
  /// be included into the solver to help run it. Can drastically change what sort of circuits are
  /// run.
  pub solver_active: bool
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
      debug_tracers: ActiveTracers::empty(),
      solver_active: false
    }
  }
}
