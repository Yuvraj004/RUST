// Import necessary modules from the `diesel` crate
use diesel::prelude::*;
use crate::schema::posts;

// Define a struct `Post` that represents a row in the `posts` table
#[derive(Queryable, Selectable)]
// Specify the table name using the `diesel` attribute macro
#[diesel(table_name = crate::schema::posts)]
// Specify the backend to use (PostgreSQL) using the `check_for_backend` attribute macro
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    // Define fields that correspond to columns in the `posts` table
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

//creating a new posts
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}