use super::helper::{ft_unwrap, get_ft};
use crate::file_shell::{
    file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
    user::access_key::AccessKey,
};

// use super::helper::get_ft_guard;

pub fn write(
    file_path: &str,
    content: String,
    ackey: AccessKey,
) -> Result<usize, FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;

    let file_ptr = ft.open(file_path)?;
    drop(ft);
    let file_result = file_ptr.as_ref().write();
    let mut file_guard = match file_result {
        Ok(s) => s,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "write:获取文件锁失败"
            )))
        }
    };
    file_guard.dir_user_check(ackey)?;
    file_guard.file_write(content)
}
