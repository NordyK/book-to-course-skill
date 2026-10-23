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

struct DraftState;

impl State for DraftState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingState { approvals: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn status(&self) -> &'static str {
        "draft"
    }
    fn can_edit(&self) -> bool {
        true
    }
}

struct PendingState {
    approvals: u32,
}

impl State for PendingState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals + 1 >= 2 {
            Box::new(PublishedState)
        } else {
            Box::new(PendingState { approvals: self.approvals + 1 })
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(DraftState)
    }
    fn status(&self) -> &'static str {
        "pending"
    }
}

struct PublishedState;

impl State for PublishedState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn status(&self) -> &'static str {
        "published"
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(DraftState)), content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_edit() {
            self.content.push_str(text);
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn status(&self) -> &'static str {
        self.state.as_ref().unwrap().status()
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
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

impl Draft {
    pub fn new() -> Draft {
        Draft { content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> Pending {
        Pending { content: self.content }
    }
}

impl Pending {
    pub fn approve(self) -> Published {
        Published { content: self.content }
    }
    pub fn reject(self) -> Draft {
        Draft { content: self.content }
    }
}

impl Published {
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests;
