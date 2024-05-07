use std::cell::RefCell;

pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}
pub struct PendingReviewPost {
    approval_count: RefCell<u8>,
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            approval_count: RefCell::new(0),
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(&self) -> Option<Post> {
        // Dereferencing here causes infinite loop
        let mut count = self.approval_count.borrow_mut();
        if *count == 3 {
            Some(Post {
                content: self.content.clone(),
            })
        } else {
            *count += 1;
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
