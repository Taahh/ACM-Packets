mod buffer;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::buffer::Buffer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {addr}");
    tokio::spawn(async move { // Client Thread Loop
        loop {
            if let Ok(mut ret) = TcpStream::connect(&addr).await {
                let mut buffer = Buffer::new();
                buffer.write_string("Hellaur");
                let _ = ret.write_all(&(buffer.data)).await;
                println!("Wrote to stream!");
                break;
            }
        }
    });
    println!("Beginning server loop.");
    // Server Loop
    loop {
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                println!("Received {n} bits");
                let mut sliced = Buffer::from((&buf[0..n]).to_vec());
                let str_len = sliced.read_string();
                println!("{str_len}");

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
