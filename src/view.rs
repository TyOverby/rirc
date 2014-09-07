use termbox_console::TermboxConsole;
use console_draw::ConsoleCanvas;
use console_draw::{
    Character,
    Special,
    Resize,
    Backspace,
    Enter
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
    console: TermboxConsole,
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
                let console = TermboxConsole::new();
                let (width, height) = (console.width(), console.height());
                Ok(View {
                    console: console,
                    con: ConWrapper::new(c),
                    input_buffer: String::new(),
                    width: width,
                    height: height
                })
            }
            Err(x) => Err(x)
        }
    }

    fn repaint(&mut self) {
        self.console.clear();

        // textbox
        let textbox_y = self.height - 2;
        self.console.draw(1, textbox_y, self.input_buffer.as_slice());
        self.console.cursor(self.input_buffer.len() + 1, textbox_y as uint);

        // Lines in the buffer
        let mut line_y = 1;
        for &line in self.con.buf.last_n_truncated(self.height - 4, self.width - 2).iter() {
            self.console.draw(1, line_y, line);
            line_y += 1;
        }
        self.console.present();
    }

    pub fn send_msg(&mut self) {
        self.input_buffer.push_char('\n');
        self.con.con.write(self.input_buffer.clone());
        self.input_buffer.clear();
    }

    pub fn update(&mut self) -> bool {
        self.console.clear();
        for event in self.console {
            match event {
                Character('q') => {
                    return false;
                }
                Character(c) => {
                    self.input_buffer.push_char(c);
                }
                Special(Backspace) => {
                    self.input_buffer.pop_char();
                }
                Special(Enter) => {
                    self.send_msg();
                }
                Resize(w, h) => {
                    self.width = w;
                    self.height = h;
                }
                _ => {  }
            }
            self.repaint();
        }

        match self.con.drink() {
            Ok(true) => self.repaint(),
            Ok(false) => {}
            Err(()) => { return false; }
        }
        true
    }
}
