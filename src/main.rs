use domain::bank_account::{BankAccount, Error};
use crate::domain::amount::{amount};
use crate::domain::amount::Amount;
mod domain;

fn main() -> Result<(), Error> {
    let mut account = BankAccount::create_new_account("0987654323".to_string(), amount!(2000))?;
    account.deposit(amount!(100));
    account.deposit(amount!(3000));
    account.withdraw(amount!(6000));


    println!("{account}");
    Ok(())
}
