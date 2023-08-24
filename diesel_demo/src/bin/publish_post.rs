// Import necessary modules and types
use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
    // Import specific items from the schema and dsl modules
    use self::schema::posts::dsl::{posts, published};

    // Get the post ID from the command line arguments
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    // Establish a connection to the database
    let connection = &mut establish_connection();

    // Update the post with the given ID and set 'published' field to true
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning()) // Specify the fields to return
        .get_result(connection)
        .unwrap(); // Get the updated post from the database

    // Print a message indicating that the post has been published
    println!("Published post {}", post.title);
    println!("Published post details: {}", post.body);
}
