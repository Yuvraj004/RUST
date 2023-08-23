
use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap_or_else(|_| panic!("Error in stRING " ));

    let post: models::Post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(windows)]
const EOF: &str = "CTRL+Z";