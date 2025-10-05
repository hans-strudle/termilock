use std::time::Duration;
use std::io::{self, Write};
use std::fs;
use crossterm::{QueueableCommand, cursor, terminal};
use std::process::Command;
use serde_json;

#[derive(Debug)]
pub struct Plugins {
    pub list: Vec<Plugin>,
    pub findDir: String,
    pub config_file: String,
}
/*
pub fn build_shell_fn(cmd: &str) -> impl Fn(&str) -> String {
    |cmd| -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("pmset -g batt");
        String::from_utf8(output.stdout).unwrap()
    }
}*/

#[derive(Debug)]
pub struct Plugin {
    // name: String,
    pub x: u16,
    pub y: u16,
    pub delay: Option<Duration>,
    pub cmd: String,
}

impl Plugins {
    pub fn load(&mut self) -> io::Result<&Vec<Plugin>> {
        let mut plugin_list: Vec<Plugin> = Vec::new(); 
        //  for file in fs::read_dir(&self.findDir)? {
        //      
        //      println!("{:?}", file);
        //  }
        let file = fs::File::open(&self.config_file)?;
        let config: serde_json::Value = serde_json::from_reader(file)?;
        // let config_plugins = config.get("plugins").expect("get plugins");
        if let Some(plugins) = config["plugins"].as_array() {
            for plugin in plugins {
                println!("{:?}", plugin);
                println!("{}", plugin["x"]);
                let x = plugin["x"].as_u64().expect("x from json");
                let y = plugin["y"].as_u64().expect("y from json");
                let cmd  = &plugin["command"].as_str().expect("command str");

                let p = Plugin {
                    // name: plugin.name,
                    // command: plugin.command,
                    delay: None,
                    x: x as u16,
                    y: y as u16,
                    cmd: cmd.to_string(),
                    // text: plugin.text,
                    // duration: None,
                };
                self.list.push(p);
                // plugin_list.push(p);
                // println!("{}", plugin_list.len());
            }
        }

        Ok(&self.list)
    }
}

impl Plugin {
   pub fn call(&self, stdout: &mut io::Stdout) -> String {
       // let ret = (self.func)();
       let output = Command::new("sh")
            .arg("-c")
            .arg(&self.cmd)
            .output()
            .expect("pmset -g batt");
       let ret = String::from_utf8(output.stdout).unwrap();
       stdout.queue(cursor::MoveTo(self.x, self.y));
       stdout.write_all(ret.as_bytes());
       // stdout.write_all(&self.cmd.as_bytes());
       // stdout.flush();
       // println!("{}", ret);
       ret
   } 
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_plugin_load_config() {
        let mut TestPlugins: Plugins = Plugins {
            config_file: "config.json".to_string(),
            findDir: "plugins/".to_string(),
            list: Vec::new(),
        };

        TestPlugins.load();
        println!("{:?}", &TestPlugins.config_file);
        println!("{:?}", &TestPlugins.list);
        assert!(TestPlugins.list.len() > 0);
    }
    #[test]
    fn test_run_plugins() {
        let mut TestPlugins: Plugins = Plugins {
            config_file: "config.json".to_string(),
            findDir: "plugins/".to_string(),
            list: Vec::new(),
        };

        TestPlugins.load();

        let mut stdout = io::stdout();
        // stdout.queue(terminal::Clear(terminal::ClearType::All));
        // stdout.flush();
        for plugin in &TestPlugins.list {
            let t = plugin.call(&mut stdout);
            println!("{}", t);
        }
    }
}
