use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::http::request::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("listening to {}", self.addr);

        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recived a request : {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {}
                                Err(e) => {}
                            }
                        }

                        Err(e) => println!("Failed to read from connection"),
                    }
                }
                Err(e) => {
                    println!("failed to established connection {} ", e)
                }
            }
        }
    }
}
