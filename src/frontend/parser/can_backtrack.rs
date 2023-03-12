use super::Result;

pub trait CanBacktrack
{
  fn breakpoint<T>(
    &mut self,
    computation: impl FnOnce(&mut Self) -> Result<T>,
  ) -> Result<T>;
}

/// Attempts a parse with backtrack breakpoint and returns immediately if
/// successful.
macro_rules! attempt {
  ($source:ident as $target:ident => $expr:expr) => {
    if let Ok(result) = $source.breakpoint(|$target| $expr) {
      return Ok(result)
    }
  };
}
