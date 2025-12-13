#[derive(Debug, Clone)]
pub struct PostContent {
    pub title: String,
    pub body: String,
}

pub struct New;
pub struct Unmoderated;
pub struct Published;
pub struct Deleted;

#[derive(Debug, Clone)]
pub struct Post<State> {
    pub content: PostContent,
    state: std::marker::PhantomData<State>,
}

impl Post<New> {
    pub fn new(title: &str, body: &str) -> Self {
        Post {
            content: PostContent {
                title: title.to_string(),
                body: body.to_string(),
            },
            state: std::marker::PhantomData,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        Post {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl Post<Deleted> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitions_work() {
        let new_post = Post::<New>::new("My Post", "Hello!");
        let unmoderated = new_post.publish();
        let published = unmoderated.allow();
        let _deleted = published.delete();
    }

    #[test]
    fn deny_works() {
        let new_post = Post::<New>::new("Draft", "Waiting for review");
        let unmoderated = new_post.publish();
        let _deleted = unmoderated.deny();
    }
}
