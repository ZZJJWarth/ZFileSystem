use std::ops::DerefMut;

use crate::file_shell::{
    file_table::file_table::FileTable,
    root_file::{error::FileSystemOperationError, root_file::VFile},
};

pub fn mkdir(current_path: &str, dir_name: &str) -> Result<(), FileSystemOperationError> {
    let mut ft = FileTable::new();
    let ls_dir = ft.open(current_path)?;

    let dir = ls_dir.as_ref().write();
    let mut dir_guard = match dir {
        Ok(dir) => dir,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "在获取文件锁时出现错误"
            )));
        }
    };
    dir_guard.dir_mkdir(dir_name);
    // let vdir=dir_guard.deref_mut();

    // let mut zdir=match vdir{
    //     VFile::ZDir(dir)=>{dir},
    //     VFile::ZFile(file)=>{return Err(FileSystemOperationError::NotDirError(format!("这里需要一个目录，但是这里却是文件")))}
    // };

    // let mut ft=FileTable::new();
    // let ls_dir=ft.open(current_path)?;
    // let mut zdir_guard=ls_dir.as_ref().write();
    // let vdir=match zdir_guard{
    //     Ok(mut a)=>a.deref_mut(),
    //     Err(_)=>return Err(FileSystemOperationError::LockError(String::from("未能成功获取锁"))),
    // };
    // let mut zdir=match vdir{
    //     VFile::ZDir(dir)=>{dir},
    //     VFile::ZFile(file)=>{return Err(FileSystemOperationError::NotDirError(format!("这里需要一个目录，但是这里却是文件")))}
    // };
    // zdir.mkdir(dir_name)?;
    Ok(())
}
