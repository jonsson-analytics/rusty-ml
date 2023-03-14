pub trait TransformInto<Representation>
{
  type Context<'a>;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Representation;
}
