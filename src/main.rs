#![crate_type = "lib"]

extern crate irc_message;
extern crate termbox_console;
extern crate console_draw;

use view::View;
use irc_con::IrcConnection;

mod irc_con;
mod irc_buffer;
mod view;

fn main() {
   let mut view = View::new("irc.mozilla.org").unwrap();

   while view.update() {}
   /*
   let irccon = IrcConnection::new("localhost", 12321).unwrap();
   irccon.write("fooooo".to_string());
   loop {
    match irccon.read() {
        Ok(m) => println!("{}", m),
        _ => { irccon.write("baaar\n".to_string()); }
    }
   }*/
}
