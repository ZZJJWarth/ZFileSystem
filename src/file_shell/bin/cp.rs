use crate::file_shell::{root_file::error::FileSystemOperationError, user::access_key::AccessKey};

use super::{
    helper::{ft_unwrap, get_ft},
    touch::touch,
};
///source是源文件的路径
///dest_path是复制文件要进入的目录的路径
///file_name是复制文件的名字
pub fn cp(
    source_path: &str,
    dest_path: &str,
    file_name: &str,
    ackey: AccessKey,
) -> Result<(), FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let file_ptr = ft.open(source_path)?;
    drop(ft);
    let file = file_ptr.as_ref().read();

    match file {
        Ok(s) => {
            let u_id = ackey.u_id;
            s.dir_user_check(ackey)?;
            s.file_cp(dest_path, file_name, u_id)
        }
        Err(_) => Err(FileSystemOperationError::LockError(format!(
            "cp:获取锁失败"
        ))),
    }
}

///host复制功能，source_path为操作系统中的文件，dest_path是本文件系统中的目录,file_name是要创建的文件名
pub fn host_cp(
    source_path: &str,
    dest_path: &str,
    file_name: &str,
    owner_u_id: u8,
) -> Result<(), FileSystemOperationError> {
    // touch(dest_path, file_name, owner_u_id)?;
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let file_ptr = ft.open(dest_path)?;
    drop(ft);
    let mut file = file_ptr.as_ref().write();

    match file {
        Ok(mut s) => s.dir_host_cp(source_path, file_name, owner_u_id),
        Err(_) => Err(FileSystemOperationError::LockError(format!(
            "cp:获取锁失败"
        ))),
    }
}
