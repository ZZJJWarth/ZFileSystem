use crate::file_shell::{
    file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
};

pub fn touch(current_path: &str, file_name: &str) -> Result<(), FileSystemOperationError> {
    let mut ft = FileTable::new();
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
    dir_guard.dir_touch(file_name)
}
