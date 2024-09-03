// use crossterm::terminal;
mod pass;
mod cli;
mod plugins;
use cli::Cli;
use clap::Parser;
use std::io::{Write, self};
use std::fmt;
use crossterm::{
    QueueableCommand,
    terminal, cursor, style::{Stylize},
    event, csi,
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
    let PASS_LENGTH;
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
    let mut plugins = Vec::new();
    let f = || -> String {
        return "LOCKED".to_string();
        // chrono::offset::Local::now().to_string()
    };
    let time_plugin = plugins::Plugin {
        x: 1,
        y: 1,
        delay: Some(Duration::from_millis(500)),
        func: f,
    };
    plugins.push(time_plugin);
    // let hashed = pass::hash_pass(&cli.pass);
    pass::set_password(&cli.pass)?;
    // pass::set_password(&cli.pass)?;
    let PASS = pass::get_password()?;
    // println!("hashed: {}, {}", hashed, pass::hash_pass("999999999999"));
    // thread::sleep(Duration::from_secs(5));
    let LOCK_STRING = "_ ".repeat(PASS_LENGTH - 1) + "_";
    terminal::enable_raw_mode()?;
    stdout.queue(terminal::SetTitle("termilock"))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    // let e = Element::new("asdf", 10, 10);
    // stdout.queue(e)?;
    // stdout.queue(Element::new("heyyyy", 10, 40))?;

    let mut bad_pass_attempt: bool = false;

    let mut quit = false;
    // let poll_duration = Duration::from_millis(500);
    let mut input = String::new();
    let star = format!("{}", "*".blue());
    while !quit {
        for plugin in &plugins {
            plugin.call(&mut stdout);
        }
        let offset = (LOCK_STRING.len() / 2) as u16;
        let mut x = 0;
        for place in 0..PASS_LENGTH {
            stdout.queue(cursor::MoveTo(width / 2 - offset + x, height / 2));
            stdout.write("_".as_bytes())?;
            stdout.queue(cursor::MoveTo(width / 2 - offset + x, height / 2));
            if input.len() > place {
                stdout.write(star.as_bytes())?;
            }
            // stdout.flush()?;
            x += 2;
        }
        if bad_pass_attempt {
            stdout.queue(cursor::MoveTo(width / 2 - 5, height / 2 + 2))?;
            let s = format!("{}", "WRONG PASS".red());
            stdout.write(s.as_bytes())?;
        }
        // reset cursor to current pass input
        let diff = (2 * input.len()) as u16;
        stdout.queue(cursor::MoveTo(width / 2 - offset + diff, height / 2))?;
        stdout.flush()?;

        //if event::poll(Duration::ZERO)? {
        match event::read()? {
            event::Event::Resize(nw, nh) => {
                width = nw;
                height = nh;
                stdout.queue(terminal::Clear(terminal::ClearType::All))?;
                stdout.flush();
            },
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Tab => {}, // skip tabs?
                    event::KeyCode::Home => {}, // skip Home?
                    event::KeyCode::Char(' ') => {}, // skip spaces
                    event::KeyCode::Backspace => {
                        if input.len() > 0 {
                            input.pop();
                        }
                    },
                    _ => {
                        if event.code == event::KeyCode::Char('c') {
                            if event.modifiers.contains(event::KeyModifiers::CONTROL){
                                quit = true;
                            }
                        }
                        input.push_str(&event.code.to_string());
                        // stdout.write(star.as_bytes());
                    }
                }
                stdout.flush();
                if pass::hash_pass(&input) == PASS {
                    // succeed
                    terminal::disable_raw_mode();
                    quit = true;
                    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
                }
                if input.len() == PASS_LENGTH {
                    // wrong password
                    // reset?
                    bad_pass_attempt = true;
                    input = "".to_string();
                    // stdout.flush()?;
                }
            },
            _ => {}
        }
        // }
        stdout.flush();
        // quit = true; 
    }
    // println!("{:?}", input);

    terminal::disable_raw_mode()?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0,0))?;
    stdout.flush()?;

    Ok(())
}
