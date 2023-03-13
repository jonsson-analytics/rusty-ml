use crate::syntax::debrujin;
use crate::transform_into::TransformInto;

struct LFV(pub usize);

pub trait LargestFreeVariable
{
  fn largest_free_variable(
    &self,
    context: usize,
  ) -> usize;
}

impl<'a, Representation> LargestFreeVariable for Representation
where
  Representation: TransformInto<LFV, Context<'a> = usize>,
{
  fn largest_free_variable(
    &self,
    context: usize,
  ) -> usize
  {
    let LFV(name) = self.transform(context);
    return name
  }
}

impl TransformInto<LFV> for debrujin::Expression
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LFV
  {
    match self {
      | debrujin::Expression::Literal(_) => LFV(0),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | debrujin::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<LFV> for debrujin::Abstraction
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LFV
  {
    self.body.transform(context + 1)
  }
}

#[cfg(test)]
mod abstraction
{
  use pretty_assertions::assert_eq;
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
    let LFV(largest) = abstraction.transform(0);
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
    let LFV(largest) = abstraction.transform(0);
    assert_eq!(largest, 1);
  }
}

impl TransformInto<LFV> for debrujin::Application
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LFV
  {
    let LFV(abstraction) = self.abstraction.transform(context);
    let LFV(argument) = self.argument.transform(context);
    LFV(std::cmp::max(abstraction, argument))
  }
}

#[cfg(test)]
mod application
{
  use pretty_assertions::assert_eq;
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
    let LFV(largest) = application.transform(0);
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
    let LFV(largest) = application.transform(0);
    assert_eq!(largest, 2);
  }
}

impl TransformInto<LFV> for debrujin::Identifier
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> LFV
  {
    LFV(match () {
      | _ if self.name < context => 0,
      | _ => self.name + 1 - context,
    })
  }
}

#[cfg(test)]
mod identifier
{
  use pretty_assertions::assert_eq;
  use super::*;

  #[test]
  fn bound()
  {
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let LFV(largest) = identifier.transform(1);
    assert_eq!(largest, 0);

    let identifier = debrujin::Identifier {
      name: 1,
    };
    let LFV(largest) = identifier.transform(2);
    assert_eq!(largest, 0);
  }

  #[test]
  fn unbound()
  {
    let identifier = debrujin::Identifier {
      name: 1,
    };
    let LFV(largest) = identifier.transform(1);
    assert_eq!(largest, 1);

    let identifier = debrujin::Identifier {
      name: 2,
    };
    let LFV(largest) = identifier.transform(1);
    assert_eq!(largest, 2);
  }
}
