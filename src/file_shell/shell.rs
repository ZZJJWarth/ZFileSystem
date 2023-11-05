use std::io::{self, stdin, Write};

use crate::file_shell::bin;

pub struct Shell {
    path: String,
    user: String,
}

impl Shell {
    pub fn new(user: &str) -> Shell {
        Shell {
            path: String::from("~/"),
            user: user.to_string(),
        }
    }

    pub fn head(&self) -> String {
        format!("\n{}:{}", self.user, self.path)
    }

    pub fn shell(&mut self, input: &mut String) -> String {
        // let mut input=String::new();

        let head = self.head();
        let content = match input.trim() {
            "EXIT" => "".to_string(),
            "ls" => bin::ls::ls(),
            "" => "".to_string(),
            _ => {
                format!("Err:there is no such a command:{}", input)
            }
        };
        format!("{}", content)
    }
}
