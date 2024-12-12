use crate::bank_account::{BankAccount, Error};

mod bank_account;

fn main() -> Result<(), Error> {
    let mut account = BankAccount::create_new_account("0987654323".to_string(), 2000)?;
    account.deposit(100);
    account.deposit(3000);
    account.withdraw(6000);

    println!("{account}");
    Ok(())
}
