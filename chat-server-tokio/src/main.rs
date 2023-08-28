use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
//by deafult rust does not know how to execute the async func so tokio
//provides that executor
#[tokio::main] //executor macro
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap(); //returns a future therfore we use await

    let (tx,mut rx) = broadcast::channel::<String>(10);

    loop {
        let (mut _sockett, _addr) = listener.accept().await.unwrap();

        //separation of read and write part required becoz Buffreader is taking owneship of the socket earlier

        //removing errors by clonign tx
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move{
            let (reader, mut writer) = _sockett.split();
        //to read and put the data which is read from stream
            let mut reader = BufReader::new(reader);

            let mut line = String::new();
            loop {
                //to store each line
                //read_line fucntion adds to the prevous line doesnot generate new one
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                writer.write_all(msg.as_bytes()).await.unwrap(); //passing the buffer to be written untill the no of bytes read
                line.clear()
            }
        });
        
    }
}
//telnet localhost 8080