extern crate termbox;
use std::io::net::tcp::TcpStream;
use std::io::{ BufferedStream, IoResult };
use std::io::TimedOut;

use termbox::{
    Normal,
    White,
    Black,
};

use std::comm::{
    Sender,
    TryRecvError,
    Receiver,
    channel,
    Empty,
    Disconnected
};

use std::task::spawn;

pub struct IrcConnection {
    pub server: String,
    pub port: u16,
    read_in: Receiver<String>,
    write_out: Sender<String>,
    stream: TcpStream
}

fn reader(stream: TcpStream) -> Receiver<String> {
    let (s, r) = channel();
    spawn(proc() {
        let sender = s;
        let stream = stream;
        let mut b_stream = BufferedStream::new(stream);
        loop {

            let line = b_stream.read_line();
            match line {
                Ok(l) => match sender.send_opt(l) {
                    Ok(()) => {},
                    Err(_) => { break; }
                },
                Err(e) => {
                    break;
                }
            }
        }
    });
    r
}

fn writer(stream: TcpStream) -> Sender<String> {
    let (s, r) = channel();
    spawn(proc() {
        let receiver = r;
        let mut stream = stream;
        loop {
            //termbox::print(20,20, Normal, White, Black, "foo");
            //termbox::present();
            let line: String = match receiver.try_recv() {
                Ok(s) => s,
                Err(Empty) => { continue; }
                Err(Disconnected) => { break; }
            };
            termbox::print(20,20, Normal, White, Black, line.as_slice());
            termbox::present();
            match stream.write_str(line.as_slice()) {
                Ok(()) => {
                    match stream.flush() {
                        Ok(()) => { },
                        Err(_) => {
                            break;
                        }
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    });
    s
}

impl IrcConnection {
    pub fn new(server: &str, port: u16) -> IoResult<IrcConnection> {
        let tcpstream = try!(TcpStream::connect(server, port));
        Ok(IrcConnection {
            server: server.to_string(),
            port: port,
            read_in: reader(tcpstream.clone()),
            write_out: writer(tcpstream.clone()),
            stream: tcpstream.clone(),
        })
    }

    pub fn write(&self, message: String) -> Result<(), String> {
        self.write_out.send_opt(message)
    }

    pub fn read(&self) -> Result<String, TryRecvError> {
        self.read_in.try_recv()
    }
}

impl Drop for IrcConnection {
    fn drop(&mut self) {
        self.stream.close_read();
        self.stream.close_write();
    }
}
