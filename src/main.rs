extern crate dci_example_rs;

use dci_example_rs::account::Account;
use dci_example_rs::transfer_source::TransferSource;

fn main() {
    let source0 = Account::new(0);
    let sink0 = Account::new(0);
    let source1 = source0.increase_balance(100000);

    let (source2, sink1) = source1.transfer_to(200, sink0);
    println!("{:?}, {:?}", source2, sink1)
}
