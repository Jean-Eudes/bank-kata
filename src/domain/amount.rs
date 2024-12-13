use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
macro_rules! amount {
    ($amount: expr) => {
        Amount::new($amount)
    };
}

pub(crate) use amount;

#[derive(Debug, PartialEq)]
pub struct Amount(pub i64);

impl Amount {
    pub fn new(amount: i64) -> Amount {
        Amount(amount)
    }
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }
}

impl Add<&Amount> for &Amount {
    type Output = Amount;

    fn add(self, rhs: &Amount) -> Self::Output {
        Amount(self.0 + rhs.0)
    }
}

impl Sub<&Amount> for &Amount {
    type Output = Amount;

    fn sub(self, rhs: &Amount) -> Self::Output {
        Amount(self.0 - rhs.0)
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{number:>width$}",
            number = &self.0,
            width = f.width().unwrap_or(0)
        )
    }
}
