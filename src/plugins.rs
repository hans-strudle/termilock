use std::time::Duration;
use std::io::{self, Write};
use std::fs;
use crossterm::{QueueableCommand, cursor};
use std::process::Command;
use serde_json;

pub struct Plugins {
    pub list: Vec<Plugin>,
    pub findDir: String,
    pub config_file: String,
}

pub fn build_shell_fn(cmd: &str) -> impl Fn(&str) -> String {
    |cmd| -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("pmset -g batt");
        String::from_utf8(output.stdout).unwrap()
    }
}

pub struct Plugin {
    // name: String,
    pub x: u16,
    pub y: u16,
    pub delay: Option<Duration>,
    pub func: fn() -> String,
}

impl Plugins {
    pub fn load(&mut self) -> io::Result<Vec<Plugin>> {
        let plugin_list = Vec::new(); 
        //  for file in fs::read_dir(&self.findDir)? {
        //      
        //      println!("{:?}", file);
        //  }
        let mut file = fs::File::open(&self.config_file)?;
        let mut config: serde_json::Value = serde_json::from_reader(file)?;
        // let config_plugins = config.get("plugins").expect("get plugins");
        if let Some(plugins) = config["plugins"].as_array() {
            for plugin in plugins {
                println!("{:?}", plugin);
                println!("{}", plugin["x"]);
                let x = plugin["x"].as_u64().expect("x from json");
                let y = plugin["y"].as_u64().expect("y from json");
                let cmd  = &plugin["command"];
                /* let p = Plugin {
                    // name: plugin.name,
                    // command: plugin.command,
                    delay: None,
                    x: x as u16,
                    y: y as u16,
                    func: build_shell_fn(&cmd.to_string()),
                    // text: plugin.text,
                    // duration: None,
                };*/
            }
        }

        Ok(plugin_list)
    }
}

impl Plugin {
   pub fn call(&self, mut stdout: &mut io::Stdout) -> String {
       let ret = (self.func)();
       stdout.queue(cursor::MoveTo(self.x, self.y));
       stdout.write(ret.as_bytes());
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
        assert!(TestPlugins.list.len() > 0);
    }
}
