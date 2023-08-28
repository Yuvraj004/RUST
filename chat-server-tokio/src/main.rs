use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
//by deafult rust does not know how to execute the async func so tokio
//provides that executor
#[tokio::main] //executor macro
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap(); //returns a future therfore we use await

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut _sockett, _addr) = listener.accept().await.unwrap();

        //removing errors by cloning tx
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move{
            let (reader, mut writer) = _sockett.split();
        //to read and put the data which is read from stream
            let mut reader = BufReader::new(reader);

            let mut line = String::new();
            loop {
                //problem: same msg is also repeating in same client
                tokio::select! {
                    result = reader.read_line(&mut line) =>{
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv()=>{
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap(); //passing the buffer to be written untill the no of bytes read
                    }
                }
            }
        });
        
    }
}
//telnet localhost 8080