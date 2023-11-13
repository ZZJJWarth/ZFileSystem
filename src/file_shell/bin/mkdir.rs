use std::ops::DerefMut;

use crate::file_shell::{
    file_table::file_table::FileTable,
    root_file::{error::FileSystemOperationError, root_file::VFile},
};

use super::helper::{ft_unwrap, get_ft};

// use super::helper::get_ft_guard;

pub fn mkdir(current_path: &str, dir_name: &str,owner_u_id:u8) -> Result<(), FileSystemOperationError> {
    // let mut ft=get_ft_guard()?;
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let ls_dir = ft.open(current_path)?;
    drop(ft);
    let dir = ls_dir.as_ref().write();
    let mut dir_guard = match dir {
        Ok(dir) => dir,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "在获取文件锁时出现错误"
            )));
        }
    };
    dir_guard.dir_mkdir(dir_name,owner_u_id);
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
