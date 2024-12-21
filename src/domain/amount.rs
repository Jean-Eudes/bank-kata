use deepsize::DeepSizeOf;
use num_traits::{AsPrimitive, PrimInt, Signed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

#[macro_export]
macro_rules! amount {
    ($amount: expr) => {
        PositiveAmount::new($amount)
    };
}

#[macro_export]
macro_rules! balance {
    ($amount: expr) => {
        Balance::new($amount)
    };
}

pub type PositiveAmount = Amount<u64>;
pub type Balance = Amount<i64>;

#[derive(Debug, PartialEq, Clone, DeepSizeOf)]
pub struct Amount<T>(T);

impl<T> Amount<T> {
    pub fn new(amount: T) -> Amount<T> {
        Amount(amount)
    }
}

impl<T> Amount<T>
where
    T: Signed,
{
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<T, U> Add<Amount<T>> for Amount<U>
where
    T: Copy + AsPrimitive<U>,
    U: Add<Output = U> + Copy + 'static,
{
    type Output = Amount<U>;

    fn add(self, rhs: Amount<T>) -> Self::Output {
        let t = self.0 + rhs.0.as_();
        Amount(t)
    }
}
impl<T, U> Sub<Amount<T>> for Amount<U>
where
    T: Copy + AsPrimitive<U>,
    U: Sub<Output = U> + Copy + 'static,
{
    type Output = Amount<U>;

    fn sub(self, rhs: Amount<T>) -> Self::Output {
        let t = self.0 - rhs.0.as_();
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
