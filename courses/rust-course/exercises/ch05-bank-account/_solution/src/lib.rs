/// A bank account. The balance can never go below zero.
pub struct Account {
    owner: String,
    balance: u64,
    history: Vec<i64>,
}

impl Account {
    /// A new, empty account.
    pub fn new(owner: &str) -> Self {
        Self { owner: String::from(owner), balance: 0, history: Vec::new() }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    /// Adds money and records +amount.
    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        self.history.push(amount as i64);
    }

    /// Takes money out if there is enough; records -amount. Returns whether it happened.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        self.history.push(-(amount as i64));
        true
    }

    /// Every change, in order.
    pub fn history(&self) -> &[i64] {
        &self.history
    }

    /// Closes the account and returns what was left.
    pub fn close(self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests;
