use blog::Post;

fn main() {
    let mut post = Post::new();

    // Rust Type Transitions
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    // State Pattern (OOP)
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.reject();
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());
}
