use crate::account::Account;

// Methodless Role
pub trait TransferSource {
    fn transfer_to(&self, amount: u64, account: Account) -> (Account, Account);
}

