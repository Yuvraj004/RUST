use mini_redis::{client, Result};

#[tokio::main]
// The async fn definition looks like a regular synchronous function, but operates asynchronously. Rust transforms the async fn at compile time into a routine that operates asynchronously. Any calls to .await within the async fn yield control back to the thread. The thread may do other work while the operation processes in the background.
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;
    //c::connect is from mini-redis , establishes a async TCP conn with 
    // a specific remote address.

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}