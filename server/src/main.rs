use std::io;
use std::{
    io::Write
};
use tokio::{
    io::{BufReader,BufWriter,AsyncBufReadExt,AsyncWriteExt},
    net::{
        TcpListener,
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf}
    }
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop{
        let (stream, addr) = listener.accept().await.unwrap();
        print!{"{addr}\n"}
        tokio::spawn(async move{handle_socket(stream).await});
    }
}

async fn handle_socket(stream: TcpStream){
    let (mut stream_read, mut stream_write) = stream.into_split();
    tokio::spawn(async move{handle_writer(&mut stream_write).await});
    tokio::spawn(async move{handle_reader(&mut stream_read).await});
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
