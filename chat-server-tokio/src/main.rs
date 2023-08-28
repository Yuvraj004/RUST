use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
#[tokio::main] 
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap(); 
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut _sockett, _addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = _sockett.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) =>{
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(),_addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv()=>{
                        let (msg,other_addr) = result.unwrap();
                        if _addr!=other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap(); 
                        }
                    }
                }
            }
        });
    }
}
//telnet localhost 8080
