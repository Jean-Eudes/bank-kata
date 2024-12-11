use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use Transaction::{Deposit, Withdraw};

type Amount = i64;

enum Transaction {
    Deposit(DateTime<Utc>, Amount),
    Withdraw(DateTime<Utc>, Amount),
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Deposit(date, amount) => {
                write!(
                    f,
                    "Deposit : '{}' {}",
                    date.format("%Y-%m-%d %H:%M:%S"),
                    amount
                )
            }
            Withdraw(date, amount) => {
                write!(
                    f,
                    "Withdraw: '{}' {}",
                    date.format("%Y-%m-%d %H:%M:%S"),
                    amount
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
    pub fn create_new_account(account_number: String, initial_amount: Amount) -> Self {
        Self {
            initial_amount,
            account_number,
            transactions: vec![],
        }
    }

    pub fn deposit(&mut self, amount: Amount) {
        let now = Utc::now();
        self.transactions.push(Deposit(now, amount));
    }

    pub fn withdraw(&mut self, amount: Amount) {
        let now = Utc::now();
        self.transactions.push(Withdraw(now, amount));
    }

    pub fn balance(&self) -> Amount {
        let mut balance = self.initial_amount;
        for transaction in &self.transactions {
            balance += match transaction {
                Deposit(_, amount) => *amount,
                Withdraw(_, amount) => -amount,
            }
        }
        balance
    }
}

impl Display for BankAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Account: {} {}", self.account_number, self.balance())?;
        for transaction in &self.transactions {
            writeln!(f, " {}", transaction)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    #[test]
    fn should_create_new_account() {
        // Given / When
        let bank_account = BankAccount::create_new_account("account_number".to_string(), 100);

        // Then
        assert_eq!(bank_account.initial_amount, 100);
        assert_eq!(bank_account.account_number, "account_number");
    }

    #[test]
    fn should_deposit_in_bank_account() {
        // Given
        let mut bank_account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        bank_account.deposit(50);

        // Then
        assert_eq!(bank_account.transactions.len(), 1);
        assert!(matches!(bank_account.transactions[0], Deposit(_, 50)));
    }

    #[test]
    fn should_withdraw_in_bank_account() {
        // Given
        let mut bank_account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        bank_account.withdraw(50);

        // Then
        assert_eq!(bank_account.transactions.len(), 1);
        assert!(matches!(bank_account.transactions[0], Withdraw(_, 50)));
    }

    #[test]
    fn should_balance_in_bank_account() {
        // Given
        let mut bank_account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        bank_account.deposit(50);
        bank_account.withdraw(100);

        // Then
        assert_eq!(bank_account.balance(), 50);
    }

    #[test]
    fn should_format_account() {
        // Given
        let mut bank_account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        bank_account.deposit(50);
        bank_account.withdraw(100);

        // Then
        let re = Regex::new(
            r"Account: account_number 50\n Deposit : '(.*)' 50\n Withdraw: '(.*)' 100\n",
        )
        .unwrap();
        assert!(re.is_match(&bank_account.to_string()));
    }
}
