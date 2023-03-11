pub trait TransformInto<Representation>
{
  type Context<'a>;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> Representation;
}
