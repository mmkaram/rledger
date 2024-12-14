mod structs;

use structs::{Account, Cash, Currency, Expense, Ledger, Transaction};
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
    let test_account_two = Account::new("account:chase");
    let test_cur_two = Currency::new("usd");
    let test_cash_two = Cash::new(100, test_cur_two);
    let test_expense_two = Expense::new("food");
    let test_transaction_two = Transaction::new(
        2,
        "Going to the plaza".to_string(),
        test_account_two,
        test_cash_two,
        test_expense_two,
    );
    let test_ledger = Ledger::new(vec![test_transaction, test_transaction_two]);
    println!("{}", test_ledger);
}
