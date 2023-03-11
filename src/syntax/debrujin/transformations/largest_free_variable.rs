use crate::syntax::debrujin;
use crate::transform_into::TransformInto;

pub struct LargestFreeVariable(pub usize);

impl TransformInto<LargestFreeVariable> for debrujin::Expression
{
  type Environment<'a> = usize;

  fn transform(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    match self {
      | debrujin::Expression::Literal(_) => LargestFreeVariable(0),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.transform(environment),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.transform(environment),
      | debrujin::Expression::Application(application) =>
        application.transform(environment),
    }
  }
}

impl TransformInto<LargestFreeVariable> for debrujin::Abstraction
{
  type Environment<'a> = usize;

  fn transform(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    self.body.transform(environment + 1)
  }
}

#[cfg(test)]
mod abstraction
{
  use super::*;

  #[test]
  fn bound()
  {
    let abstraction = debrujin::Abstraction {
      body: debrujin::Identifier {
        name: 0,
      }
      .into(),
    };
    let LargestFreeVariable(largest) = abstraction.transform(0);
    assert_eq!(largest, 0);
  }

  #[test]
  fn unbound()
  {
    let abstraction = debrujin::Abstraction {
      body: debrujin::Identifier {
        name: 1,
      }
      .into(),
    };
    let LargestFreeVariable(largest) = abstraction.transform(0);
    assert_eq!(largest, 1);
  }
}

impl TransformInto<LargestFreeVariable> for debrujin::Application
{
  type Environment<'a> = usize;

  fn transform(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    let LargestFreeVariable(abstraction) = self.abstraction.transform(environment);
    let LargestFreeVariable(argument) = self.argument.transform(environment);
    LargestFreeVariable(std::cmp::max(abstraction, argument))
  }
}

#[cfg(test)]
mod application
{
  use super::*;

  #[test]
  fn abstraction_is_greater()
  {
    let application = debrujin::Application {
      abstraction: debrujin::Identifier {
        name: 1,
      }
      .into(),
      argument: debrujin::Identifier {
        name: 0,
      }
      .into(),
    };
    let LargestFreeVariable(largest) = application.transform(0);
    assert_eq!(largest, 2);
  }

  #[test]
  fn argument_is_greater()
  {
    let application = debrujin::Application {
      abstraction: debrujin::Identifier {
        name: 0,
      }
      .into(),
      argument: debrujin::Identifier {
        name: 1,
      }
      .into(),
    };
    let LargestFreeVariable(largest) = application.transform(0);
    assert_eq!(largest, 2);
  }
}

impl TransformInto<LargestFreeVariable> for debrujin::Identifier
{
  type Environment<'a> = usize;

  fn transform(
    &self,
    environment: Self::Environment<'_>,
  ) -> LargestFreeVariable
  {
    LargestFreeVariable(match () {
      | _ if self.name < environment => 0,
      | _ => self.name + 1 - environment,
    })
  }
}

#[cfg(test)]
mod identifier
{
  use super::*;

  #[test]
  fn bound()
  {
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let LargestFreeVariable(largest) = identifier.transform(1);
    assert_eq!(largest, 0);

    let identifier = debrujin::Identifier {
      name: 1,
    };
    let LargestFreeVariable(largest) = identifier.transform(2);
    assert_eq!(largest, 0);
  }

  #[test]
  fn unbound()
  {
    let identifier = debrujin::Identifier {
      name: 1,
    };
    let LargestFreeVariable(largest) = identifier.transform(1);
    assert_eq!(largest, 1);

    let identifier = debrujin::Identifier {
      name: 2,
    };
    let LargestFreeVariable(largest) = identifier.transform(1);
    assert_eq!(largest, 2);
  }
}
