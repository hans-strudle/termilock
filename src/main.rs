// use crossterm::terminal;
mod pass;
mod cli;
use cli::Cli;
use clap::{Parser, Subcommand};
use std::io::{Write, self};
use std::fs;
use std::io::prelude::*;
use std::fmt;
use std::env;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize},
    event, csi
};
use std::time::Duration;
use std::thread;

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
    let cli = Cli::parse();
    let MIN_PASS_LENGTH = 4;
    let mut PASS_LENGTH;
    match cli.length {
        Some(pass_length) => {
            PASS_LENGTH = cli.length.unwrap();
            if pass_length != cli.pass.len() {
                panic!("Pass length must match passcode");
            }
            if pass_length < MIN_PASS_LENGTH {
                panic!("Minimum passcode length is 4!");
            } 
        },
        None => {
            // No explicit passcode length, determine based 
            // off of provided passcode
            PASS_LENGTH = cli.pass.len();
        }
    }

    let (mut width, mut height) = terminal::size()?;
    let mut stdout = io::stdout();

    // let hashed = pass::hash_pass(&cli.pass);
    pass::set_password(&cli.pass)?;
    // pass::set_password(&cli.pass)?;
    let PASS = pass::get_password()?;
    // println!("hashed: {}, {}", hashed, pass::hash_pass("999999999999"));
    println!("PASSS");
    println!("{}", PASS);
    thread::sleep(Duration::from_secs(5));
    let LOCK_STRING = "_ ".repeat(PASS_LENGTH - 1) + "_";
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
        let offset = (LOCK_STRING.len() / 2) as u16;
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
           stdout.queue(cursor::MoveTo(width / 2 - 5, height / 2 + 2));
           let s = format!("{}", "WRONG PASS".red());
           stdout.write(s.as_bytes());
           let diff = (2 * input.len()) as u16;
           stdout.queue(cursor::MoveTo(width / 2 - offset + diff, height / 2));
           stdout.flush();
        }
        match event::read()? {
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
                if event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    println!("Ctrl+C!");
                    terminal::disable_raw_mode();
                    quit = true;
                }
                if pass::hash_pass(&input) == PASS {
                    // succeed
                    terminal::disable_raw_mode();
                    quit = true;
                    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
                }
                if input.len() == PASS_LENGTH {
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

    terminal::disable_raw_mode()?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0,0))?;
    stdout.flush()?;

    Ok(())
}
