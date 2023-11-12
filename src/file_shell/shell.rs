use std::{
    fmt::format,
    io::{self, stdin, Write},
    slice::SliceIndex,
};

use crate::file_shell::{bin, root_file::root_file::RawRootFile};

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
        let cm=match command.get(0){
            Some(s)=>s,
            None=>return format!("")
        };
        let content = match cm.as_str() {
            "EXIT" => "".to_string(),
            "ls" => {
                let output = bin::ls::ls(&self.path);
                match output {
                    Ok(s) => s,
                    Err(e) => format!("{:?}", e),
                }
            }
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
                match command.get(1){
                    Some(name)=>{
                        let output = bin::mkdir::mkdir(&self.path, name.as_str());
                        match output {
                            Ok(_) => String::new(),
                            Err(e) => format!("{:?}", e,),
                        }
                    },
                    None=>{
                        format!("mkdir <Filename>:缺乏参数 <Filename>")
                    }
                }
                
            }
            "touch" => {
                match command.get(1){
                    Some(name)=>{
                        let output = bin::touch::touch(&self.path, name.as_str());
                        match output {
                            Ok(_) => String::new(),
                            Err(e) => format!("{:?}", e,),
                        }
                    },
                    None=>{
                        format!("mkdir <Filename>:缺乏参数 <Filename>")
                    }
                }
            }
            "cat" => {
                let mut path = RawRootFile::parse_path(&self.path);
                match command.get(1){
                    Some(s)=>{
                        path.push(s.clone());
                        let file_path = bin::cd::from_vec_to_path(path);
                        let output = bin::cat::cat(file_path.as_str());
                        match output {
                            Ok(s) => s,
                            Err(e) => format!("{:?}", e),
                        }
                    },
                    None=>{
                        format!("cat <File>:缺乏参数 <File>")
                    }
                }
                
            }
            "write" => {
                match command.get(1){
                    Some(path1)=>{
                        let mut path = RawRootFile::parse_path(&self.path);
                        path.push(path1.clone());
                        let file_path = bin::cd::from_vec_to_path(path);
                        match command.get(2){
                            Some(content)=>{
                                // let content = command.get(2).unwrap();
                                let output = bin::write::write(file_path.as_str(), content.clone());
                                match output {
                                    Ok(s) => {
                                        format!("{s} 字节被写入")
                                    }
                                    Err(e) => {
                                        format!("{:?}", e)
                                    }
                                }
                            },
                            None=>{
                                format!("write <File> <Content>:缺乏参数 Content")
                            }
                        }
                    },
                    None=>{
                        format!("write <File> <Content>:缺乏参数 <File>")
                    }
                }
                
                
            }
            "debug" => {
                let mut path = RawRootFile::parse_path(&self.path);
                path.push(command.get(1).unwrap().clone());
                let file_path = bin::cd::from_vec_to_path(path);
                let output = bin::debug::debug(&file_path);
                match output {
                    Ok(s) => s,
                    Err(e) => {
                        format!("{:?}", e)
                    }
                }
            }
            "cp"=>{
                
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
