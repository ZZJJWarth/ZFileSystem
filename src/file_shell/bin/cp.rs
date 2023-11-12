use crate::file_shell::root_file::error::FileSystemOperationError;

use super::helper::{get_ft, ft_unwrap};
///source是源文件的路径
///dest_path是复制文件要进入的目录的路径
///file_name是复制文件的名字
pub fn cp(source_path:&str,dest_path:&str,file_name:&str)->Result<(),FileSystemOperationError>{
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let file_ptr=ft.open(source_path)?;
    drop(ft);
    let file=file_ptr.as_ref().read();

    match file{
        Ok(s)=>{
            s.file_cp(dest_path,file_name)
        },
        Err(_)=>{
            Err(FileSystemOperationError::LockError(format!("cp:获取锁失败")))
        }
    }
}