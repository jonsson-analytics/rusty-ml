use crate::syntax::debrujin;
use crate::transform_into::TransformInto;

pub struct LargestFreeVariable(pub usize);

impl TransformInto<LargestFreeVariable> for debrujin::Expression
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LargestFreeVariable
  {
    match self {
      | debrujin::Expression::Literal(_) => LargestFreeVariable(0),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | debrujin::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<LargestFreeVariable> for debrujin::Abstraction
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LargestFreeVariable
  {
    self.body.transform(context + 1)
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
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LargestFreeVariable
  {
    let LargestFreeVariable(abstraction) = self.abstraction.transform(context);
    let LargestFreeVariable(argument) = self.argument.transform(context);
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
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LargestFreeVariable
  {
    LargestFreeVariable(match () {
      | _ if self.name < context => 0,
      | _ => self.name + 1 - context,
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
