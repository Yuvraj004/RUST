// Import necessary modules from the `diesel` crate
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Import the `dotenvy` crate to load environment variables from a `.env` file
use dotenvy::dotenv;

// Import the `env` module to access environment variables
use std::env;
pub mod models;
pub mod schema;
// Function to establish a connection to the PostgreSQL database
pub fn establish_connection() -> PgConnection {
    // Load environment variables from a `.env` file (if available)
    dotenv().ok();

    // Read the `DATABASE_URL` environment variable or panic if not set
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Attempt to establish a connection to the database
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewPost,Post};

pub fn create_post(conn: &mut PgConnection, title : &str,body: &str) -> Post {
    use crate::schema::posts;
    let new_post = NewPost { title,body};

    let inserted_post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");
    
    inserted_post
}