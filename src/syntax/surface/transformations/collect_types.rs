use crate::syntax::surface::{
  self,
  types,
};
use crate::transform_into::TransformInto;

pub struct Context
{
  stack: Vec<types::Variable>,
  free_name: usize,
}

impl Context
{
  pub fn free_name(&mut self) -> types::Variable
  {
    let free_name = self.free_name;
    self.free_name += 1;
    return types::Variable::Unnamed(free_name)
  }

  fn with_bindings<Result>(
    &mut self,
    bindings: &[types::Variable],
    computation: impl FnOnce(&mut Self) -> Result,
  ) -> Result
  {
    for binding in bindings {
      self.stack.push(binding.clone());
    }
    let result = computation(self);
    for _ in bindings {
      self.stack.pop();
    }
    return result
  }
}

impl Default for Context
{
  fn default() -> Self
  {
    Self {
      stack: Vec::new(),
      free_name: 0,
    }
  }
}

pub trait CollectType<'a>
{
  fn collect_type(
    &self,
    context: &'a mut Context,
  ) -> types::Type;
}

impl<'a, Representation> CollectType<'a> for Representation
where
  Representation: TransformInto<types::Type, Context<'a> = &'a mut Context>,
{
  fn collect_type(
    &self,
    context: &'a mut Context,
  ) -> types::Type
  {
    self.transform(context)
  }
}

impl TransformInto<types::Type> for surface::Expression
{
  type Context<'a> = &'a mut Context;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> types::Type
  {
    todo!()
  }
}
