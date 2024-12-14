mod structs;
use structs::{Account, Cash, Currency, Expense, Transaction};
fn main() {
    let test_account = Account::new("account:chase");
    let test_cur = Currency::new("usd");
    let test_cash = Cash::new(100, test_cur);
    let test_expense = Expense::new("food");
    let test_transaction = Transaction::new(
        1,
        "Going to the store".to_string(),
        test_account,
        test_cash,
        test_expense,
    );
    println!("{}", test_transaction);
}
