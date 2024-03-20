use std::panic;
use std::panic::AssertUnwindSafe;

/// Because we are primarily interfaced with from Python we want to make sure we don't panic
/// and kill the external process. This is also important because we have errors that occur in places
/// that [`Results`] cannot be used, and when we fail other systems should then respond.
///
/// This just wraps the lambda call in [`panic::catch_unwind`] and folds the panic message into
/// the expected error.
pub fn catch_panics<R, F: FnOnce() -> Result<R, String>>(wrapped: F) -> Result<R, String> {
  let result = panic::catch_unwind(AssertUnwindSafe(wrapped));
  match result {
    Ok(thread_result) => {
      if let Ok(result) = thread_result {
        Ok(result)
      } else {
        Err(thread_result.err().unwrap())
      }
    }
    Err(panic_value) => Err(
      if let Some(message) = panic_value.downcast_ref::<String>() {
        message.clone()
      } else if let Some(message) = panic_value.downcast_ref::<&str>() {
        message.to_string()
      } else {
        "Unavailable error message.".to_string()
      })
  }
}

#[cfg(test)]
mod tests {
  use crate::exceptions::catch_panics;

  #[test]
  fn success() {
    let result = catch_panics(|| Ok("dave"));

    assert!(result.is_ok() && result.ok().unwrap() == "dave")
  }

  #[test]
  fn panic() {
    let result: Result<(), String> = catch_panics(|| panic!("Ahhh?!"));

    assert!(result.is_err() && result.err().unwrap().as_str() == "Ahhh?!")
  }

  #[test]
  fn error() {
    let result: Result<(), String> = catch_panics(|| Err("Eh.".into()));

    assert!(result.is_err() && result.err().unwrap().as_str() == "Eh.")
  }
}
