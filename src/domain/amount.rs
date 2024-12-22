use deepsize::DeepSizeOf;
use num_traits::{AsPrimitive, Signed};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_positive_amount() {
        // Given / When
        let amount = PositiveAmount::new(1000);

        // Then
        assert_eq!(amount.0, 1000);
    }

    #[test]
    fn should_create_balance() {
        // Given / When
        let balance = Balance::new(1000);

        // Then
        assert_eq!(balance.0, 1000);
    }

    #[test]
    fn should_add_two_positive_amount() {
        // Given
        let amount1 = amount!(500);
        let amount2 = amount!(6000);

        // When
        let result = amount1 + amount2;

        // Then
        assert_eq!(result.0, 6500);
    }

    #[test]
    fn should_add_positive_amount_to_balance() {
        // Given
        let balance = balance!(1000);
        let amount = amount!(6000);

        // When
        let result = balance + amount;

        // Then
        assert_eq!(result.0, 7000);
    }

    #[test]
    fn should_add_balance_to_positive_amount() {
        // Given
        let balance = balance!(1000);
        let amount = amount!(6000);

        // When
        let result = amount + balance;

        // Then
        assert_eq!(result.0, 7000);
    }
    #[test]
    fn should_subtract_two_positive_amount() {
        // Given
        let amount1 = amount!(6000);
        let amount2 = amount!(500);

        // When
        let result = amount1 - amount2;

        // Then
        assert_eq!(result.0, 5500);
    }

    #[test]
    fn should_subtract_positive_amount_to_balance() {
        // Given
        let balance = balance!(1000);
        let amount = amount!(6000);

        // When
        let result = balance - amount;

        // Then
        assert_eq!(result.0, -5000);
    }

    #[test]
    fn should_subtract_balance_to_positive_amount() {
        // Given
        let balance = balance!(1000);
        let amount = amount!(6000);

        // When
        let result = amount - balance;

        // Then
        assert_eq!(result.0, 5000);
    }
}
