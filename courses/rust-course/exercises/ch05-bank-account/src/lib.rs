/// A bank account. The balance can never go below zero.
pub struct Account {
    owner: String,
    balance: u64,
    history: Vec<i64>,
}

impl Account {
    /// A new, empty account.
    pub fn new(owner: &str) -> Self {
        todo!()
    }

    pub fn owner(&self) -> &str {
        todo!()
    }

    pub fn balance(&self) -> u64 {
        todo!()
    }

    /// Adds money and records +amount.
    pub fn deposit(&mut self, amount: u64) {
        todo!()
    }

    /// Takes money out if there is enough; records -amount. Returns whether it happened.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        todo!()
    }

    /// Every change, in order.
    pub fn history(&self) -> &[i64] {
        todo!()
    }

    /// Closes the account and returns what was left.
    pub fn close(self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests;
