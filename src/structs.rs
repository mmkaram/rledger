use std::fmt;

pub struct Currency {
    name: String,
}

pub struct Cash {
    amount: i32,
    currency: Currency,
}

pub struct Account {
    name: String,
}

pub struct Expense {
    name: String,
}

pub struct Transaction {
    name: String,
    id: i32,
    from: Account,
    value: Cash,
    to: Expense,
}

impl Currency {
    pub fn new(name: &str) -> Currency {
        Currency {
            name: name.to_string(),
        }
    }
}

impl Cash {
    pub fn new(amount: i32, currency: Currency) -> Cash {
        Cash { amount, currency }
    }
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: name.to_string(),
        }
    }
}

impl Expense {
    pub fn new(name: &str) -> Expense {
        Expense {
            name: name.to_string(),
        }
    }
}

impl Transaction {
    pub fn new(id: i32, name: String, from: Account, value: Cash, to: Expense) -> Transaction {
        Transaction {
            id,
            name,
            from,
            value,
            to,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Cash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction details:\nName: {}\nid: {}\nFrom (Account): {}\nValue (Cash): {}\nTo (Expense): {}",
            self.name, self.id, self.from, self.value, self.to
        )
    }
}