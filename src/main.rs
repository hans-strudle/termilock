// use crossterm::terminal;
use std::io::{Write, self};
use std::fs;
use std::io::prelude::*;
use std::fmt;
// use crate::{csi, impl_display, Command};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize},
    event, csi
};
use std::time::Duration;
use std::thread;

const LOCK_STRING: &str = "_ _ _ _";
// const PASS: [&str; 4] = ["1","2","3","4"];
const PASS: &str = "1234";

struct Element {
    x: u16,
    y: u16,
    text: String,
}

impl Element {
    fn new(text: &str, x: u16, y: u16) -> Self {
        Element {
            x,
            y,
            text: text.to_string(),
        }
    }
}

impl crossterm::Command for Element {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{};{}H"), self.x + 1, self.y + 1);
        write!(f, "{}", self.text)
    }
}

fn main() -> io::Result<()> {
    let (mut width, mut height) = terminal::size()?;
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    stdout.queue(terminal::SetTitle("termilock"))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    // let e = Element::new("asdf", 10, 10);
    // stdout.queue(e)?;
    // stdout.queue(Element::new("heyyyy", 10, 40))?;
    let mut is_entering: bool = false;
    let mut bad_pass_attempt: bool = false;

    let mut quit = false;
    // let poll_duration = Duration::from_millis(500);
    // let mut input = Vec::new();
    let mut input = String::new();
    while !quit {
        let offset = LOCK_STRING.len() as u16;
        if !is_entering { 
            stdout.queue(cursor::MoveTo(1, 1));
            // stdout.write("🔒".as_bytes());

            stdout.queue(cursor::MoveTo(width / 2 - offset, height / 2));
            stdout.write(LOCK_STRING.as_bytes());
            stdout.queue(cursor::MoveTo(width / 2 - offset, height / 2));
            stdout.flush();
        }
        if bad_pass_attempt {
           // stdout.queue(Element::new("WRONG PASSCODE", width / 2 - 8, height / 2))?;
           stdout.queue(cursor::MoveTo(width / 2 - offset - 1, height / 2 + 2));
           let s = format!("{}", "WRONG PASS".red());
           stdout.write(s.as_bytes());
           let diff = (2 * input.len()) as u16;
           stdout.queue(cursor::MoveTo(width / 2 - offset + diff, height / 2));
           stdout.flush();
        }
        match event::read().unwrap() {
            event::Event::Key(event) => {
                // println!("{:?}", event);
                // input.push(event.code.to_string());
                is_entering = true;
                input.push_str(&event.code.to_string());
                // let star = format!("{}", "*".green().on_grey());
                let star = "*";
                stdout.write(star.as_bytes());
                let diff = (2 * input.len()) as u16;
                stdout.queue(cursor::MoveTo(width / 2 - offset + diff, height / 2));
                stdout.flush();
                /*if event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    println!("Ctrl+C!");
                    terminal::disable_raw_mode();
                    quit = true;
                }*/
                if input == PASS {
                    // succeed
                    terminal::disable_raw_mode();
                    quit = true;
                    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
                }
                if input.len() == PASS.len() {
                    // wrong password
                    // reset?
                    // stdout.queue(terminal::Clear(terminal::ClearType::All));
                    bad_pass_attempt = true;
                    is_entering = false;
                    input = "".to_string();
                    // stdout.flush()?;
                }
            },
            _ => {}
        }
        // quit = true; 
    }
    // println!("{:?}", input);

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.flush()?;
    terminal::disable_raw_mode();

    Ok(())
    }
/* 
    fn main2() {
        let mut file = fs::File::open("res/lock.txt").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        terminal::enable_raw_mode();
        let mut stdout = io::stdout();
        let (mut width, mut height) = terminal::size().unwrap();

        stdout.queue(terminal::Clear(terminal::ClearType::All));
        stdout.queue(cursor::MoveTo(1, 1));
        // stdout.write(s.as_bytes());
        stdout.write("🔒".as_bytes());

        stdout.queue(cursor::MoveTo(width / 2 - 5, height / 2));
        stdout.write(LOCK_STRING.as_bytes());
        stdout.queue(cursor::MoveTo(width / 2 - 5, height / 2));
        stdout.flush();
        let mut quit = false;
        let poll_duration = Duration::from_millis(500);
        let mut input = Vec::new();
        while !quit {
            // match event::poll(poll_duration).unwrap() {

            match event::read().unwrap() {
                event::Event::Key(event) => {
                    // println!("{:?}", event);
                    input.push(event.code.to_string());
                    stdout.write("*".as_bytes());
                    let mut diff = (2 * input.len()) as u16;
                    stdout.queue(cursor::MoveTo(width / 2 - 5 + diff, height / 2));
                    stdout.flush();
                    if event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        println!("Ctrl+C!");
                        terminal::disable_raw_mode();
                        quit = true;
                    }
                    if input == PASS {
                        terminal::disable_raw_mode();
                        quit = true;
                    }
                },
                _ => {}
            }
            // quit = true; 
        }
        println!("{:?}", input);

        }
*/
