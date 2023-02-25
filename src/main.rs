use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener};

fn main() -> io::Result<()> {
    let mut buffer = [0; 17];
    let listener = TcpListener::bind("13.228.225.19:80")?;

    for i in listener.incoming() {
        match i {
            Ok(mut stream) => {
                stream.write(b"A [QB] Broadcast.");
            }
            Err(_) => {
                println!("FAILED.");
            }
        }
     }
    
    Ok(())
}



