use crate::account::Account;

// Methodless Role
pub trait TransferSink {
    fn transfer_from(&self, amount: u64, src: &Account) -> Account;
}

