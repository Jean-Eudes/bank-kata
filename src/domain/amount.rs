use num_traits::{PrimInt, Signed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};
use deepsize::DeepSizeOf;

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

#[derive(Debug, PartialEq)]
#[derive(DeepSizeOf)]
pub struct Amount<T>(T);

impl<T: PrimInt> Amount<T> {
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

/*impl Add<&PositiveAmount> for &Balance {
    type Output = Balance;

    fn add(self, rhs: &PositiveAmount) -> Self::Output {
        let t = self.0 + rhs.0 as i64;
        Amount(t)
    }
}
*/

impl<T, U> Add<&Amount<T>> for &Amount<U>
where
    T: Add<Output=T> + Copy + Default,
    U: Add<Output=U> + Copy + TryFrom<T> + Default,
{
    type Output = Amount<U>;

    fn add(self, rhs: &Amount<T>) -> Self::Output {
        let t = self.0 + U::try_from(rhs.0).unwrap_or_else(|_| panic!("pas bien, ne devrait pas arriver"));
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
