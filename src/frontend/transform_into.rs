pub trait TransformInto<Syntax>
{
  type Environment;
  type Result<T>;

  fn encode(
    &self,
    environment: &mut Self::Environment,
  ) -> Self::Result<Syntax>;
}
