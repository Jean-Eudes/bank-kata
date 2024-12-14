use crate::domain::amount::{amount, balance, Balance};
use crate::domain::amount::PositiveAmount;
use domain::bank_account::{BankAccount, Error};

mod domain;

fn main() -> Result<(), Error> {
    let mut y: Box<[BankAccount]> = Vec::with_capacity(1000).into_boxed_slice();
    let mut account = BankAccount::create_new_account("0987654323".to_string(), balance!(2000))?;
    account.deposit(amount!(100));
    account.deposit(amount!(3000));
    account.withdraw(amount!(6000));

    println!("{account}");
    y[1] = account;
    println!("{}", y.len());
    println!("{}", y[0]);
    Ok(())
}
