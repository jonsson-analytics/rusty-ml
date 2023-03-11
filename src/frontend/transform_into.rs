pub trait TransformInto<Representation>
{
  type Environment<'a>;

  fn encode<'a>(
    &self,
    environment: Self::Environment<'a>,
  ) -> Representation;
}
