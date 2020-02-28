use chrono::{DateTime, Utc};
use crate::transfer_sink::TransferSink;
use crate::transfer_source::TransferSource;

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Account {
    available_balance: u64,
}

impl Account {
    pub fn new(balance: u64) -> Self {
        Account { available_balance: balance }
    }

    pub fn get_balance(&self) -> u64 {
        self.available_balance
    }

    pub fn increase_balance(&self, amount: u64) -> Self {
        Self::new(self.available_balance + amount)
    }

    pub fn decrease_balance(&self, amount: u64) -> Self {
        Self::new(self.available_balance - amount)
    }

    pub fn update_log(msg: String, date: DateTime<Utc>, amount: u64) {
        println!("Account {:?}, {:?}, {:?}", msg, date, amount)
    }
}

impl TransferSink for Account {
    // Methodfull Role
    fn transfer_from(&self, amount: u64, _src: &Account) -> Account {
        let result = self.increase_balance(amount);
        Account::update_log("Transfer in".to_string(), Utc::now(), amount);
        result
    }
}

impl TransferSource for Account {
    // Methodfull Role
    fn transfer_to(&self, amount: u64, account: Account) -> (Account, Account) {
        let new_source = self.decrease_balance(amount);
        Account::update_log("Transfer out".to_string(), Utc::now(), amount);
        let result = account.transfer_from(amount, self);
        (new_source, result)
    }
}

