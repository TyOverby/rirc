extern crate termbox;

use termbox::{
    Normal,
    White,
    Black,
    Enter,
    Backspace,
    Backspace2,
    Space
};

use std::io::IoResult;
use std::comm::{
    Empty,
    Disconnected
};

use super::irc_con::IrcConnection;
use super::irc_buffer::LineBuffer;

struct ConWrapper {
    con: IrcConnection,
    buf: LineBuffer
}

impl ConWrapper {
    fn new(con: IrcConnection) -> ConWrapper {
        ConWrapper { con: con, buf: LineBuffer::new() }
    }

    fn drink(&mut self) -> Result<bool, ()> {
        let mut updated = false;
        loop {
            match self.con.read() {
                Ok(m) => {
                    self.buf.add(m);
                    updated = true;
                }
                Err(Empty) => {
                    return Ok(updated);
                }
                Err(Disconnected) => {
                    return Err(());
                }
            }
        }
    }
}

pub struct View {
    con: ConWrapper,
    input_buffer: String,
    width: uint,
    height: uint
}

impl View {
    pub fn new(server: &str) -> IoResult<View> {
        let con = IrcConnection::new(server, 6666);
        //let con = IrcConnection::new("localhost", 12321);
        match con {
            Ok(c) => {
                termbox::init();
                Ok(View {
                    con: ConWrapper::new(c),
                    input_buffer: String::new(),
                    width: termbox::width(),
                    height: termbox::height()
                })
            }
            Err(x) => Err(x)
        }
    }

    fn repaint(&mut self) {
        termbox::clear();

        // textbox
        let textbox_y = self.height - 2;
        termbox::print(1, textbox_y, Normal, White, Black, self.input_buffer.as_slice());
        termbox::set_cursor(self.input_buffer.len() + 1, textbox_y as uint);

        // Lines in the buffer
        let mut line_y = 1;
        for &line in self.con.buf.last_n_truncated(self.height - 4, self.width - 2).iter() {
            termbox::print(1, line_y, Normal, White, Black, line);
            line_y += 1;
        }
        termbox::present();
    }

    pub fn send_msg(&mut self) {
        self.input_buffer.push_char('\n');
        self.con.con.write(self.input_buffer.clone());
        self.input_buffer.clear();
    }

    pub fn update(&mut self) -> bool {
        termbox::clear();
        match termbox::peek_event(0) {
            termbox::KeyEvent(_, Some(Space), _) => {
                self.input_buffer.push_char(' ');
                self.repaint();
            }
            termbox::KeyEvent(_, Some(Enter), _) => {
                self.send_msg();
                self.repaint();
            }
            termbox::KeyEvent(_, Some(Backspace), _) |
            termbox::KeyEvent(_, Some(Backspace2), _) => {
                self.input_buffer.pop_char();
                self.repaint();
            }
            termbox::KeyEvent(_, _, Some('q')) => { return false; }
            termbox::KeyEvent(_, _, Some(c)) => {
                self.input_buffer.push_char(c);
                self.repaint();
            },
            termbox::ResizeEvent(w, h) => {
                self.width = w as uint;
                self.height = h as uint;
                self.repaint();
            }
            termbox::NoEvent => {}
            termbox::KeyEvent(a,b,c) => {
                let foo = format!("{},{},{}", a, b, c);
                assert!(false);
                termbox::print(5,10, Normal, White, Black, foo.as_slice());
            }
        }

        match self.con.drink() {
            Ok(true) => self.repaint(),
            Ok(false) => {}
            Err(()) => { return false; }
        }
        true

    }
}

impl Drop for View {
    fn drop(&mut self) {
        termbox::shutdown();
    }
}
