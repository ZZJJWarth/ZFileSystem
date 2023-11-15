use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    ops::Add,
    thread,
    time::Duration,
};

use crate::file_shell::bin;

use super::{
    shell::{self, Shell},
    user::UserManager,
};

pub fn handle_shell_command(mut stream: TcpStream) {
    let user_manager = UserManager::new();

    let name = get_user_name(&mut stream);
    let u_id = match user_manager.find_user(&name) {
        Some(u_id) => u_id,
        None => {
            let mut answer = format!(
                "{name} is not in the user list, please contact with manager:@SCUT Zeng Jun "
            );
            println!("Illegal user:{name} is trying to access file system");
            stream.write_all(answer.add("\n").as_bytes());
            stream.flush().unwrap();
            return;
        }
    };
    let mut shell = Shell::new(name.as_str(), u_id);

    loop {
        {
            let mut answer = shell.head();
            answer.push('\n');
            // stream.write(answer.as_bytes());
            // stream.flush();
            // let note=String::from("input your user name:\n");
            println!("sending head:{}", answer);

            stream.write_all(answer.add("\n").as_bytes());
            stream.flush().unwrap();
        }
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next();
        let request_line = match request_line {
            Some(s) => s,
            None => {
                bin::check::check();
                println!("a shell disconnect!");
                break;
            }
        };
        let mut request_line = match request_line {
            Ok(s) => s,
            Err(_) => {
                println!("failed to read request line!");
                break;
            }
        };
        println!("{request_line}");
        let mut answer = shell.shell(request_line);
        println!("{answer}");
        // stream.write_all(answer.as_bytes());
        stream.write_all(answer.add("\n").as_bytes());
        stream.flush().unwrap();
    }
}

pub fn get_user_name(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    buf_reader.lines().next().unwrap().unwrap()
}
