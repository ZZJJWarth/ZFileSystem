use crate::{
    file_shell::{file_table::file_table::FileTable, root_file::error::FileSystemOperationError},
    SUPER_BLOCK,
};

use super::helper::{ft_unwrap, get_ft};

pub fn cat(file_path: &str) -> Result<String, FileSystemOperationError> {
    // let mut ft = FileTable::new();
    let temp = get_ft()?;

    let ft = temp.lock(); //为什么这里一定要引用

    let mut ft = ft_unwrap(ft)?;

    let dir_ptr = ft.open(file_path)?;

    let dir_result = dir_ptr.as_ref().read();

    let dir_guard = match dir_result {
        Ok(x) => x,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!(
                "cat:获取文件锁时出错"
            )));
        }
    };

    dir_guard.file_cat()
}
