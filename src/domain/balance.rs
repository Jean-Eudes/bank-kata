use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
macro_rules! balance {
    ($balance: expr) => {
        Balance::new($balance)
    };
}

use crate::domain::amount::Amount;
pub(crate) use balance;

#[derive(Debug, PartialEq)]
pub struct Balance(i64);

impl Balance {
    pub fn new(amount: i64) -> Balance {
        Balance(amount)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }
}

impl Add<&Amount> for &Balance {
    type Output = Balance;

    fn add(self, rhs: &Amount) -> Self::Output {
        Balance(self.0 + rhs.0)
    }
}

impl Sub<&Amount> for &Balance {
    type Output = Balance;

    fn sub(self, rhs: &Amount) -> Self::Output {
        Balance(self.0 - rhs.0)
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{number:>width$}",
            number = &self.0,
            width = f.width().unwrap_or(0)
        )
    }
}
