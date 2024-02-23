use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub index: i64
}

impl Qubit {
  pub fn new(index: i64) -> Qubit {
        Qubit { index }
    }

  pub fn debug(&self) -> String {
    format!("qb[{}]", self.index)
  }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str( format!("qb[{}]", self.index).as_str())
    }
}

impl PartialEq for Qubit {
  fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Qubit {}

impl Hash for Qubit {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_i64(self.index)
  }
}
