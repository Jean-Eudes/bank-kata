use crate::bank_account::{amount, Amount, BankAccount, Error};

mod bank_account;

fn main() -> Result<(), Error> {
    let mut account = BankAccount::create_new_account("0987654323".to_string(), amount!(2000))?;
    account.deposit(amount!(100));
    account.deposit(amount!(3000));
    account.withdraw(amount!(6000));

    println!("{account}");
    Ok(())
}
