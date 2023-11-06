use std::io::{self, stdin, Write};

use crate::file_shell::bin;

pub struct Shell {
    path: String,
    user: String,
}

impl Shell {
    pub fn new(user: &str) -> Shell {
        Shell {
            path: String::from("/"),
            user: user.to_string(),
        }
    }

    pub fn head(&self) -> String {
        format!("\n{}:{}>", self.user, self.path)
    }

    pub fn shell(&mut self, input: String) -> String {
        // let mut input=String::new();
        let command = Self::parse_command(&input);
        let head = self.head();
        let content = match command.get(0).unwrap().as_str() {
            "EXIT" => "".to_string(),
            "ls" => bin::ls::ls(&self.path),
            "cd" => {
                let after = bin::cd::cd(&self.path, &input);
                match after {
                    Ok(s) => {
                        self.path = s;
                        String::new()
                    }
                    Err(e) => format!("{:?}", e),
                }
            }
            "mkdir" => {
                let output = bin::mkdir::mkdir(&self.path, command.get(1).unwrap().as_str());
                match output {
                    Ok(_) => String::new(),
                    Err(e) => format!("{:?}", e),
                }
            }
            "" => "".to_string(),
            _ => {
                format!("Err:there is no such a command:{}", input)
            }
        };
        format!("{}", content)
    }

    pub fn parse_command(input: &String) -> Vec<String> {
        let mut temp = String::new();
        let mut vec = Vec::new();
        let mut start = false;
        for i in input.chars() {
            if !start && i == ' ' {
                continue;
            } else if start && i == ' ' {
                if temp.len() > 0 {
                    vec.push(temp);
                    temp = String::new();
                }
                continue;
            } else if !start && i != ' ' {
                start = true;
                temp.push(i);
            } else {
                temp.push(i);
            }
        }
        if temp.len() > 0 {
            vec.push(temp);
        }
        vec
    }
}
