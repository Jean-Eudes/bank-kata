use std::fmt::{Display, Formatter};
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
