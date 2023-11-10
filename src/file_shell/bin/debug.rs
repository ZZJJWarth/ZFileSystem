use crate::file_shell::{
    file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
};

use super::helper::{ft_unwrap, get_ft};

// use super::helper::get_ft_guard;

pub fn debug(file_path: &str) -> Result<String, FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    let file_ptr = ft.open(file_path)?;
    let file_result = file_ptr.as_ref().read();
    let file_guard = match file_result {
        Ok(s) => s,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "debug：未能获取文件锁"
            )));
        }
    };
    Ok(format!("{:?}", file_guard))
}
