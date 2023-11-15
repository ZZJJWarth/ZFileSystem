#![allow(unused)]

use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use file_shell::thread_function::handle_shell_command;
use file_shell::thread_pool::ThreadPool;
use file_shell::user::UserManager;
use sys_utility::config::config::FILE_PATH;
use sys_utility::super_block;
use sys_utility::super_block::super_block::SuperBlock;

mod file_shell;
mod sys_utility;
mod test;

static mut SUPER_BLOCK: Option<Arc<Mutex<SuperBlock>>> = None;

fn main() -> std::io::Result<()> {
    match SuperBlock::init_main(FILE_PATH) {
        Ok(_) => {
            println!("init main success");
        }
        Err(e) => {
            println!("init main error:{:?}", e);
        }
    }
    SuperBlock::init_rootdir();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => {
                println!("failed to connect!");
                continue;
            }
        };

        // println!("{}",request_line);
        let note = String::from("input your user name:\n");

        stream.write_all(note.as_bytes()).unwrap();
        pool.execute(|| handle_shell_command(stream));
    }

    Ok(())
}
