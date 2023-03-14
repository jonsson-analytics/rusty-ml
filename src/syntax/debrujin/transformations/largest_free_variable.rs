use crate::syntax::debrujin;
use crate::transform_into::TransformInto;

struct Lfv(pub usize);

pub trait LargestFreeVariable
{
  fn largest_free_variable(
    &self,
    context: usize,
  ) -> usize;
}

impl<'a, Representation> LargestFreeVariable for Representation
where
  Representation: TransformInto<Lfv, Context<'a> = usize>,
{
  fn largest_free_variable(
    &self,
    context: usize,
  ) -> usize
  {
    let Lfv(name) = self.transform(context);
    name
  }
}

impl TransformInto<Lfv> for debrujin::Expression
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Lfv
  {
    match self {
      | debrujin::Expression::Literal(_) => Lfv(0),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | debrujin::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<Lfv> for debrujin::Abstraction
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Lfv
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
    let Lfv(largest) = abstraction.transform(0);
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
    let Lfv(largest) = abstraction.transform(0);
    assert_eq!(largest, 1);
  }
}

impl TransformInto<Lfv> for debrujin::Application
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Lfv
  {
    let Lfv(abstraction) = self.abstraction.transform(context);
    let Lfv(argument) = self.argument.transform(context);
    Lfv(std::cmp::max(abstraction, argument))
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
    let Lfv(largest) = application.transform(0);
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
    let Lfv(largest) = application.transform(0);
    assert_eq!(largest, 2);
  }
}

impl TransformInto<Lfv> for debrujin::Identifier
{
  type Context<'a> = usize;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Lfv
  {
    Lfv(match () {
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
    let Lfv(largest) = identifier.transform(1);
    assert_eq!(largest, 0);

    let identifier = debrujin::Identifier {
      name: 1,
    };
    let Lfv(largest) = identifier.transform(2);
    assert_eq!(largest, 0);
  }

  #[test]
  fn unbound()
  {
    let identifier = debrujin::Identifier {
      name: 1,
    };
    let Lfv(largest) = identifier.transform(1);
    assert_eq!(largest, 1);

    let identifier = debrujin::Identifier {
      name: 2,
    };
    let Lfv(largest) = identifier.transform(1);
    assert_eq!(largest, 2);
  }
}
