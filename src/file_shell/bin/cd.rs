use std::{fmt::format, iter::FlatMap, ops::Deref};
// use clap::Parser;

use crate::file_shell::{
    file_table::file_table::FileTable,
    root_file::{
        error::FileSystemOperationError,
        root_file::{RawRootFile, VFile},
    },
};

use super::helper::{ft_unwrap, get_ft};

// use super::helper::get_ft_guard;
///输入当前路径和cd命令，如果可以成功，那么返回成功后的路径。如果失败就返回失败信息
pub fn cd(shell_path: &String, command: &str) -> Result<String, FileSystemOperationError> {
    let next_file = match get_args(command) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    let current_path = shell_path.as_str();
    let after_path = to_next_file(current_path, next_file); //这个是cd后的文件路径

    // let path=RawRootFile::parse_path(current_path);
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let ls_dir = ft.open(&after_path)?; //尝试打开文件，如果失败就返回错误，如果成功就返回cd后的路径
    let dir = ls_dir.as_ref().read();
    let mut dir = match dir {
        Ok(dir) => dir,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "在获取文件锁时出现错误"
            )));
        }
    };
    let dir = dir.deref();
    match dir {
        VFile::ZDir(_) => {}
        VFile::ZFile(_) => {
            return Err(FileSystemOperationError::NotDirError(format!(
                "这不是一个目录:{command}"
            )))
        }
    }

    Ok(after_path)
    // let dir=ls_dir.as_ref().read();
    // let mut dir=match dir{
    //     Ok(dir)=>{dir},
    //     Err(_)=>{return Err(FileSystemOperationError::LockError(String::from("在获取文件锁时出现错误")));}
    // };
    // let dir=dir.deref();
}

fn get_args(command: &str) -> Result<String, FileSystemOperationError> {
    let mut ans = String::new();
    let mut flag = false;
    let mut end = false;
    let mut iter = command.trim().chars();
    iter.next();
    iter.next();
    for i in iter {
        if i == ' ' && !flag {
            //用于处理cd后面的空格
            continue;
        } else if i != ' ' && !flag {
            flag = true;
            ans.push(i);
        } else {
            if (i == '/' || i == ' ') {
                end = true;
                continue;
            } else if end && i != ' ' {
                return Err(FileSystemOperationError::TooManyArgumentsError(
                    String::from(format!("向cd输入了过多的参数:{command}")),
                ));
            }

            ans.push(i);
        }
        // ans.push(i);
    }
    Ok(ans)
}

pub fn from_vec_to_path(vec: Vec<String>) -> String {
    let mut ans = String::new();
    ans.push('/');
    for i in vec {
        if i.len() == 0 {
            continue;
        }
        if i == ".." {
            if ans.len() == 1 {
                continue;
            }
            ans.pop();
            loop {
                let now = ans.pop().unwrap();
                // println!("now={}",now);
                if now == '/' {
                    ans.push(now);
                    break;
                }
            }
        } else if i == "." {
            continue;
        } else {
            ans.push_str(&i);
            ans.push('/');
        }
    }
    // ans.pop();
    ans
}

fn to_next_file(current_path: &str, next_file: String) -> String {
    let mut now = RawRootFile::parse_path(current_path);
    now.push(next_file);
    from_vec_to_path(now)
}
#[cfg(test)]
#[test]
fn test_args() {
    // println!("{:?}",get_args("cd .. ").unwrap());
    let mut a = RawRootFile::parse_path("/hi/howareyou/");
    let b = get_args("cd no").unwrap();
    a.push(String::from(b));
    // a.push(String::from(".."));
    // println!("{:?}",a);
    let a = from_vec_to_path(a);
    println!("{:?}", a);
}
