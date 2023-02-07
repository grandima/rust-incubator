use std::marker::PhantomData;

mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

#[derive(Clone)]
struct Post {
    pub id: post::Id,
    pub user_id: user::Id,
    pub title: post::Title,
    pub body: post::Body,
}
struct Sender<S> {
  pub inner: Post,
  pub state: PhantomData<S>,
}
struct SenderNew;
struct SenderUnmoderated;
struct SenderPublished;
struct SenderDeleted;

impl Sender<SenderNew> {
    fn publish(mut self) -> Sender<SenderUnmoderated> {
        Sender { inner: self.inner, state: PhantomData }
    }
}

impl Sender<SenderUnmoderated> {
    fn allow(mut self) -> Sender<SenderPublished> {
        Sender { inner: self.inner, state: PhantomData }
    }
    fn deny(mut self) -> Sender<SenderDeleted> {
        Sender { inner: self.inner, state: PhantomData }
    }
}

impl Sender<SenderPublished> {
    fn delete(mut self) -> Sender<SenderDeleted> {
        Sender { inner: self.inner, state: PhantomData }
    }
}
#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;
    #[test]
    fn test_post() {
        let mut post = Post{id: post::Id(1), user_id: user::Id(1), title: post::Title("AAA".to_string()), body: post::Body("AAA".to_string())};
        let new: Sender<SenderNew> = Sender{inner: post, state: PhantomData};
        //cannot call deny on new post. compile error
        //new.deny()
        //can only publish a new post
        let unmoderated = new.publish();
        //can successfuly retreieve a post
        post = unmoderated.inner;
        assert!(post.id.0 == 1);
    }
}

fn main() {
    
    println!("Implement me!");
}
