use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread::JoinHandle,
};

pub struct Server {}

impl Server {
    /*fn new() -> Self {
        Self {}
    }*/
    pub fn start(self: Arc<&'static Self>) {
        let mut handle_vec: Vec<JoinHandle<()>> = Vec::new();
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            let stream_ref = self.clone();
            match stream {
                Ok(stream) => {
                    let join_handle = std::thread::spawn(|| stream_ref.handle(stream));
                    handle_vec.push(join_handle);
                }
                Err(e) => {
                    println!("error {}", e);
                }
            }
        }
        for handle in handle_vec {
            //if thread paniced nothing should happen
            let _join_res = handle.join();
        }
    }

    pub fn handle(self: Arc<&Self>, mut stream: TcpStream) {
        let mut buf: Vec<u8> = Vec::new();
        let res = Read::read_to_end(&mut stream, &mut buf);
        match res {
            Ok(_) => {
                println!("{:?}", buf);
            }
            Err(e) => {
                println!("failed to read, what: {}", e);
            }
        }
    }
}
