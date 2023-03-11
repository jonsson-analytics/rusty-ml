pub trait TransformInto<Representation>
{
  type Environment<'a>;

  fn transform<'a>(
    &self,
    environment: Self::Environment<'a>,
  ) -> Representation;
}
