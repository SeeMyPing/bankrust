mod account;

use account::entity::Account;
fn main() {
    let account = Account::create(1, "Alice".to_string());
    println!("{:?}", account.display());
}
