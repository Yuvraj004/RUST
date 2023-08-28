use tokio::net::TcpListener;
//by deafult rust does not know how to execute the async func so tokio 
//provides that executor
#[tokio::main] //executor macro
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();//returns a future therfore we use await

    let (socket,_addr) = listener.accept().await.unwrap();
}
