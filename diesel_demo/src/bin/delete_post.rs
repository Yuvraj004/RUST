// Import necessary modules and types
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main(){
    // Import specific items from the schema and dsl modules
    use self::schema::posts::dsl::*;

    // Get the target pattern from command line arguments
    let target = args()
        .nth(1)
        .expect("Expected a target to match against");

    // Create a pattern with wildcard (%) characters
    let pattern = format!("%{}%", target);

    // Establish a connection to the database
    let conn = &mut establish_connection();

    // Delete posts where the title matches the specified pattern
    let num_del = diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn) // Execute the delete operation
        .expect("Error deleting");

    // Print the number of posts deleted
    println!("Deleted {} posts", num_del);
}
