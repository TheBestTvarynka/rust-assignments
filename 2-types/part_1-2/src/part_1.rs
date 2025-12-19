use std::marker::PhantomData;

pub struct New;
pub struct Unmoderated;
pub struct Published;
pub struct Deleted;

pub struct Post<State> {
    content: String,
    _state: PhantomData<State>,
}
impl Post<New> {
    pub fn new(content: String) -> Self {
        Self {
            content,
            _state: PhantomData,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            content: self.content,
            _state: PhantomData,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            content: self.content,
            _state: PhantomData,
        }
    }
    pub fn deny(self) -> Post<Deleted> {
        Post {
            content: self.content,
            _state: PhantomData,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            content: self.content,
            _state: PhantomData,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_and_allow() {
        let post = Post::new("hello".into());
        let post = post.publish();
        let post = post.allow();
        assert_eq!(post.content(), "hello");
    }

    #[test]
    fn deny_post() {
        let post = Post::new("test".into());
        let post = post.publish();
        let _deleted = post.deny();
    }
}
