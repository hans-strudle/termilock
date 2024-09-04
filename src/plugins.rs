use std::time::Duration;
use std::io::{self, Write};
use crossterm::{QueueableCommand, cursor};

pub struct Plugin {
    // name: String,
    pub x: u16,
    pub y: u16,
    pub delay: Option<Duration>,
    pub func: fn() -> String,
}

impl Plugin {
   pub fn call(&self, mut stdout: &mut io::Stdout) -> String {
       let ret = (self.func)();
       stdout.queue(cursor::MoveTo(self.x, self.y));
       stdout.write(ret.as_bytes());
       ret
   } 
}
