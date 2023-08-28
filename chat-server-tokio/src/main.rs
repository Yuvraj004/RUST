use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
//by deafult rust does not know how to execute the async func so tokio 
//provides that executor
#[tokio::main] //executor macro
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();//returns a future therfore we use await

    let (mut _sockett,_addr) = listener.accept().await.unwrap();
    //to read and put the data which is read from stream
    let mut buffer = [0u8,255];

    let bytes_read = _sockett.read(&mut buffer).await.unwrap();

    _sockett.write_all(&buffer[..bytes_read]).await.unwrap();//passing the buffer to be written untill the no of bytes read
}
