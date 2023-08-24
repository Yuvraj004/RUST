//TO RUN THIS FILE WE USE "cargo run --bin write_post"
use diesel_demo::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    // println!(
    //     "\nOk! Let's write {} (Press {} when finished)\n",
    //     title, EOF
    // );
    // stdin()
    //     .read_to_string(&mut body)
    //     .expect("Failed to read input");
    println!(
        "\nOk! Let's write {} (Type an empty line to finish)\n",
        title
    );

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let trimmed = buffer.trim();

        if trimmed.is_empty() {
            break;
        }

        body.push_str(trimmed);
        body.push('\n'); // Add newline since read_line trims it
    }
    
    let post: models::Post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";
