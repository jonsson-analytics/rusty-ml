use super::feedable_result::FeedableResult;

pub trait Feedable
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult;
}
