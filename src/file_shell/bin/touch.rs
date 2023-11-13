use super::helper::{ft_unwrap, get_ft};
use crate::file_shell::{
    file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
};

// use super::helper::get_ft_guard;

pub fn touch(current_path: &str, file_name: &str,owner_u_id:u8) -> Result<(), FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let lock_ptr = ft.open(current_path)?;
    let dir_result = lock_ptr.as_ref().write();
    let mut dir_guard = match dir_result {
        Ok(x) => x,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(String::from(
                "touch:获取当前目录锁时失败",
            )))
        }
    };
    dir_guard.dir_touch(file_name,owner_u_id)
}
