// use std::io::prelude::*;
use std::io;
use std::io::Write;
use tokio::{
    io::{BufWriter,BufReader,AsyncWriteExt,AsyncBufReadExt},
    net::{
        TcpStream, 
        tcp::{OwnedWriteHalf, OwnedReadHalf},
    }
};
// use tokio::net::TcpListener

#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    let stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    let (mut stream_read ,mut stream_write) = stream.into_split();
    // join!(handle_writer(&mut stream_write), handle_reader(&mut stream_read));
    tokio::spawn(async move{handle_writer(&mut stream_write).await});
    tokio::spawn(async move{handle_reader(&mut stream_read).await});
    loop{}
    Ok(())
}

async fn handle_writer(stream : &mut OwnedWriteHalf){
    let mut buffer_write = BufWriter::new(stream);
    loop{
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("failed");
        // if message.trim() == "end_stream"{
        //     let _ = buffer_write.write_all(format!("{message}").as_bytes()).await;
        //     let _ = buffer_write.flush().await;
        //     stream.shutdown();
        //     break;
        // }
        let _ = buffer_write.write_all(format!("{message}").as_bytes()).await;
        // let l = buffer_write.buffer();
        // let len = l.len();
        // println!("len: {len}");
        let _ = buffer_write.flush().await;
    }
}

async fn handle_reader(stream : &mut OwnedReadHalf){
    let mut buffer_reader = BufReader::new(stream);
    loop{
        let mut line = String::new();
        let _ = buffer_reader.read_line(&mut line).await;
        print!("{line}");
        io::stdout().flush().unwrap();
    }
}
