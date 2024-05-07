mod state_lib;
mod type_lib;

fn main() {
    // State library
    let mut post = state_lib::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    // Removing this will not cause a compile error but will compile and run and panic on line 21
    // because the approve will not change the state because it is no longer in PendingReview state
    post.request_review();

    post.approve();
    post.approve(); // Second approval required
    assert_eq!("I ate a salad for lunch today", post.content());

    // ---------------------------------------------------------
    // Type library
    let mut post = type_lib::Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    let post = post.request_review();
    // assert_eq!("", post.content());

    let post = post.reject();
    let post = post.request_review();

    let mut approved_post = post.approve();
    while let None = approved_post {
        println!("Not approved yet");
        approved_post = post.approve();
    }

    println!("Approved");
    assert_eq!(
        "I ate a salad for lunch today",
        approved_post.expect("Not approved").content()
    );
}
