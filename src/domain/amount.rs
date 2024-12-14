use num_traits::{Signed, ToPrimitive};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

macro_rules! amount {
    ($amount: expr) => {
        PositiveAmount::new($amount)
    };
}

pub(crate) use amount;
macro_rules! balance {
    ($amount: expr) => {
        Balance::new($amount)
    };
}

pub(crate) use balance;

pub type PositiveAmount = Amount<u64>;
pub type Balance = Amount<i64>;

#[derive(Debug, PartialEq)]
pub struct Amount<T: ToPrimitive>(T);

impl<T: ToPrimitive> Amount<T> {
    pub fn new(amount: T) -> Amount<T> {
        Amount(amount)
    }
}

impl<T> Amount<T>
where
    T: Signed + ToPrimitive,
{
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl Add<&PositiveAmount> for &Balance {
    type Output = Balance;

    fn add(self, rhs: &PositiveAmount) -> Self::Output {
        let t = self.0 + rhs.0 as i64;
        Amount(t)
    }
}
impl Sub<&PositiveAmount> for &Balance {
    type Output = Balance;

    fn sub(self, rhs: &PositiveAmount) -> Self::Output {
        let t = self.0 - rhs.0 as i64;
        Amount(t)
    }
}

impl<T> Display for Amount<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{number:>width$}",
            number = &self.0,
            width = f.width().unwrap_or(0)
        )
    }
}
