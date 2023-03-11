use crate::frontend::transform_into::TransformInto;
use crate::frontend::debrujin;

pub struct LargestFreeVariable(usize);

impl TransformInto<LargestFreeVariable> for debrujin::Identifier
{
  type Environment<'a> = usize;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    LargestFreeVariable(match () {
      | _ if self.name < environment => 0,
      | _ => self.name - environment,
    })
  }
}

impl TransformInto<LargestFreeVariable> for debrujin::Expression
{
  type Environment<'a> = usize;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    match self {
      | debrujin::Expression::Literal(_) => LargestFreeVariable(0),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.encode(environment),
      | debrujin::Expression::Identifier(_) => LargestFreeVariable(0),
      | debrujin::Expression::Abstraction(abstraction) => todo!(),
      | debrujin::Expression::Application(application) => todo!(),
    }
  }
}