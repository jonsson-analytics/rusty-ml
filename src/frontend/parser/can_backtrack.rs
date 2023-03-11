use super::Result;

pub trait CanBacktrack
{
  fn breakpoint<T>(
    &mut self,
    computation: impl FnOnce(&mut Self) -> Result<T>,
  ) -> Result<T>;
}
