use crate::domain::amount::Amount;
use crate::domain::bank_account::Error::AccountFundCanBePositive;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use Transaction::{Deposit, Withdraw};

#[derive(Debug)]
pub enum Error {
    AccountFundCanBePositive(Amount),
}
enum Transaction {
    Deposit(DateTime<Utc>, Amount, Amount),
    Withdraw(DateTime<Utc>, Amount, Amount),
}

impl Transaction {
    fn balance(&self) -> &Amount {
        match self {
            Deposit(_, _, balance) => balance,
            Withdraw(_, _, balance) => balance,
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Deposit(date, amount, balance) => {
                write!(
                    f,
                    "{} || {:9} || {:9} || {:7}",
                    date.format("%Y/%m/%d"),
                    "",
                    amount,
                    balance,
                )
            }
            Withdraw(date, amount, balance) => {
                write!(
                    f,
                    "{} || {:9} || {:9} || {:7}",
                    date.format("%Y/%m/%d"),
                    amount,
                    "",
                    balance,
                )
            }
        }
    }
}

pub struct BankAccount {
    account_number: String,
    initial_amount: Amount,
    transactions: Vec<Transaction>,
}

impl BankAccount {
    pub fn create_new_account(
        account_number: String,
        initial_amount: Amount,
    ) -> Result<Self, Error> {
        if initial_amount.is_negative() {
            return Err(AccountFundCanBePositive(initial_amount));
        }
        Ok(Self {
            initial_amount,
            account_number,
            transactions: vec![],
        })
    }

    pub fn deposit(&mut self, amount: Amount) {
        let now = Utc::now();
        let balance = self.balance() + &amount;
        self.transactions.push(Deposit(now, amount, balance));
    }

    pub fn withdraw(&mut self, amount: Amount) {
        let now = Utc::now();
        let balance = self.balance() - &amount;
        self.transactions.push(Withdraw(now, amount, balance));
    }

    pub fn balance(&self) -> &Amount {
        self.transactions
            .last()
            .map_or(&self.initial_amount, |t| t.balance())
    }
}

impl Display for BankAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "account number {}", self.account_number)?;
        writeln!(f, " Date        || credit   || debit     || balance")?;
        for transaction in &self.transactions {
            writeln!(f, " {transaction}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::amount::amount;
    use regex::Regex;

    #[test]
    fn should_create_new_account() {
        // Given / When
        let bank_account =
            BankAccount::create_new_account("account_number".to_string(), amount!(100)).unwrap();

        // Then
        assert_eq!(bank_account.initial_amount, amount!(100));
        assert_eq!(bank_account.account_number, "account_number");
    }

    #[test]
    fn should_deposit_in_bank_account() {
        // Given
        let mut bank_account =
            BankAccount::create_new_account("account_number".to_string(), amount!(100)).unwrap();

        // When
        bank_account.deposit(amount!(50));

        // Then
        assert_eq!(bank_account.transactions.len(), 1);
        assert!(matches!(bank_account.transactions[0], Deposit(_, _, _)));
        assert_eq!(bank_account.transactions[0].balance(), &amount!(150));
    }

    #[test]
    fn should_withdraw_in_bank_account() {
        // Given
        let mut bank_account =
            BankAccount::create_new_account("account_number".to_string(), Amount(100)).unwrap();

        // When
        bank_account.withdraw(amount!(50));

        // Then
        assert_eq!(bank_account.transactions.len(), 1);
        assert!(matches!(bank_account.transactions[0], Withdraw(_, _, _)));
    }

    #[test]
    fn should_balance_in_bank_account() {
        // Given
        let mut bank_account =
            BankAccount::create_new_account("account_number".to_string(), amount!(100)).unwrap();

        // When
        bank_account.deposit(amount!(50));
        bank_account.withdraw(amount!(100));

        // Then
        assert_eq!(bank_account.balance(), &amount!(50));
    }

    #[test]
    fn should_format_account() {
        // Given
        let mut bank_account =
            BankAccount::create_new_account("account_number".to_string(), amount!(100)).unwrap();

        // When
        bank_account.deposit(amount!(50));
        bank_account.withdraw(amount!(100));

        // Then
        let re = Regex::new(
            r" Date       || credit   || debit     || balance
                   .* ||          ||        50 ||     150
                   .* ||      100 ||            ||      50
",
        )
        .unwrap();
        assert!(re.is_match(&bank_account.to_string()));
    }
}
