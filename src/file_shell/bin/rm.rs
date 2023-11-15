use crate::file_shell::{root_file::error::FileSystemOperationError, user::access_key::AccessKey};

use super::helper::{ft_unwrap, get_ft};
///给定一个目录和文件名，删除对应的文件
pub fn rm(
    dir_path: &str,
    file_name: &str,
    ackey: AccessKey,
) -> Result<(), FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;

    let f = ft.open(dir_path)?;

    drop(ft);
    let f_result = f.write();

    let mut dir = match f_result {
        Ok(dir) => dir,
        Err(e) => return Err(FileSystemOperationError::LockError(format!("获取锁失败"))),
    };
    dir.dir_user_check(ackey)?;
    dir.dir_rm(file_name)
}
