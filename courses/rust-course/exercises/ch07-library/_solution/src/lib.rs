pub mod catalog {
    pub struct Item {
        pub title: String,
        copies: u32,
    }

    impl Item {
        pub fn new(title: &str, copies: u32) -> Item {
            Item { title: String::from(title), copies }
        }

        pub fn copies(&self) -> u32 {
            self.copies
        }

        /// Removes one copy if there is one. Only this crate may call it.
        pub(crate) fn take(&mut self) -> bool {
            if self.copies == 0 {
                return false;
            }
            self.copies -= 1;
            true
        }
    }
}

pub mod loans {
    use crate::catalog::Item;

    pub fn borrow(item: &mut Item) -> Result<(), String> {
        if item.take() {
            Ok(())
        } else {
            Err(format!("no copies of {}", item.title))
        }
    }
}

pub fn stock_report(items: &[catalog::Item]) -> String {
    let mut lines = Vec::new();
    for item in items {
        lines.push(format!("{}: {}", item.title, item.copies()));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests;
