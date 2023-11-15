use std::ops::{Deref, DerefMut};

use crate::file_shell::{
    file_table::file_table::FileTable,
    root_file::{error::FileSystemOperationError, root_file::VFile},
};

use super::helper::{ft_unwrap, get_ft};

// use super::helper::get_ft_guard;

pub fn ls(path: &str) -> Result<String, FileSystemOperationError> {
    // todo!()
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;

    // let mut ft=match ft{
    //     Ok(ft)=>ft,
    //     Err(e)=>return format!("{:?}",e),
    // };
    let ls_dir = ft.open(path).unwrap();

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
    // println!("{:?}",dir);
    let ans = match dir {
        VFile::ZDir(ref zdir) => zdir.ls(),
        VFile::ZFile(ref file) => {
            return Err(FileSystemOperationError::NotDirError(format!(
                "输入的路径指向一个文件，但是ls指令应该是目录！"
            )));
        }
    };
    Ok(ans)
}

pub fn ls_l(path:&str)->Result<String,FileSystemOperationError>{
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let ls_dir = ft.open(path).unwrap();

    let dir = ls_dir.as_ref().read();
    let mut dir = match dir {
        Ok(dir) => dir,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "在获取文件锁时出现错误"
            )));
        }
    };

    dir.dir_ls_l()
}

#[cfg(test)]
#[test]
fn test_ls() {
    let ans = ls("/warth1");
    // println!("{}", ans);
}
