use blog::StatePost;
use blog::Post;

fn main() {
    let mut state_post = StatePost::new();

    state_post.add_text("I ate a salad for lunch today");
    assert_eq!("", state_post.content());

    state_post.request_review();
    assert_eq!("", state_post.content());

    state_post.approve();
    assert_eq!("", state_post.content());
    state_post.approve();

    assert_eq!("I ate a salad for lunch today", state_post.content());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}