use std::mem;
use deepsize::DeepSizeOf;
use crate::domain::amount::Balance;
use crate::domain::amount::PositiveAmount;
use domain::bank_account::{BankAccount, Error};

mod domain;

#[derive(DeepSizeOf)]
pub struct BankAccount2 {
    account_number: [u8; 9],
    account_number2: u32,
    initial_amount: Balance,
}
#[derive(DeepSizeOf)]
pub struct BankAccount4([u8; 9], [u8; 9], Balance);
#[derive(DeepSizeOf)]
pub struct BankAccount3 {
    account_number: String,
    initial_amount: Balance,
}


macro_rules! string_to_array {
    ($s:expr) => {{
        let chars: Vec<char> = $s.chars().collect();
        if chars.len() != 9 {
            panic!("La chaîne doit contenir exactement 9 caractères.");
        }
        let mut array = [0u8; 9];
        for (i, c) in chars.into_iter().enumerate() {
            array[i] = c as u8 - 0x30;
        }
        array
    }};
}

fn main() -> Result<(), Error> {
    let mut y: Box<[BankAccount]> = Vec::with_capacity(1000).into_boxed_slice();
    let mut account = BankAccount::create_new_account("0987654323".to_string(), balance!(2000))?;
    account.deposit(amount!(100));
    account.deposit(amount!(3000));
    account.withdraw(amount!(6000));
    account.withdraw(amount!(6000));

    println!("{account}");
    let mut y = [3; 4];
    y[1] = 1;
    println!("{}", y.len());
    println!("{}", y[0]);
    println!("{}", y[1]);
    let option = y.get_mut(5);
    let mut vec1 = vec![1];
    vec1.push(2);
    let my_string = "1234567890";
    //let my_array = string_to_array!(my_string);
    let my_array = string_to_array!("1234567a9");
    println!("{:?}", my_array); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("{:?}", mem::size_of::<BankAccount>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("{:?}", mem::size_of::<Balance>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("{}", mem::size_of_val(&account)); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("{}", mem::size_of_val(&account)); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec u8: {}", mem::size_of::<BankAccount2>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    let account2 = BankAccount2 { account_number: [0u8; 9], initial_amount: balance!(3), account_number2: 923456789 };
    let account3 = BankAccount3 { account_number: "123456789".to_string(), initial_amount: balance!(3) };
    let account4 = BankAccount4 ([0u8; 9], [1u8;9], balance!(4));
    println!("avec u8: {}", account2.deep_size_of()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec balance: {}", mem::size_of::<Balance>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec tab8: {}", mem::size_of::<[u8; 9]>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec u32: {}", mem::size_of::<u32>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec u32: {}", u32::MAX); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec str: {}", account3.deep_size_of()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("avec 4: {}", account4.deep_size_of()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    let x1 = "openid mail".to_string();
    println!("avec 4: {:?}", x1.split(" ").collect::<Vec<&str>>()); // Output: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
    println!("{:?}", i64::try_from(1));
    Ok(())
}
