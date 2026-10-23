// Part 1: state objects
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn status(&self) -> &'static str;
    fn can_edit(&self) -> bool {
        false
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// TODO: DraftState, PendingState { approvals: u32 }, PublishedState

impl Post {
    pub fn new() -> Post {
        todo!()
    }
    pub fn add_text(&mut self, text: &str) {
        todo!()
    }
    pub fn content(&self) -> &str {
        todo!()
    }
    pub fn status(&self) -> &'static str {
        todo!()
    }
    pub fn request_review(&mut self) {
        todo!()
    }
    pub fn approve(&mut self) {
        todo!()
    }
    pub fn reject(&mut self) {
        todo!()
    }
}

// Part 2: type-state
pub struct Draft {
    content: String,
}
pub struct Pending {
    content: String,
}
pub struct Published {
    content: String,
}

// TODO: impl Draft { new, add_text, request_review }
// TODO: impl Pending { approve, reject }
// TODO: impl Published { content }

#[cfg(test)]
mod tests;
