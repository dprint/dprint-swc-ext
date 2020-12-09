use std::cell::Ref;
use swc_common::comments::{SingleThreadedComments, SingleThreadedCommentsMapInner};

pub struct CommentContainer<'a> {
  leading: Ref<'a, SingleThreadedCommentsMapInner>,
  trailing: Ref<'a, SingleThreadedCommentsMapInner>,
}

impl<'a> CommentContainer<'a> {
  pub fn new(comments: &'a SingleThreadedComments) -> Self {
    let (leading, trailing) = comments.borrow_all();
    CommentContainer { leading, trailing }
  }
}
