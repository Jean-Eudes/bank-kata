use crate::bank_account::BankAccount;

mod bank_account;

fn main() {
    let mut account = BankAccount::create_new_account("0987654323".to_string(), 2000);
    account.deposit(100);
    account.deposit(300);
    account.withdraw(150);

    println!("{account}");
}
