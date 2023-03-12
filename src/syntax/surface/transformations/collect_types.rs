use crate::syntax::surface::{
  self,
  types,
};
use crate::transform_into::TransformInto;

pub struct Context
{
}

impl Default for Context
{
  fn default() -> Self
  {
    Self {
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
