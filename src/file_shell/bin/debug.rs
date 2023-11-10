use crate::file_shell::{
    file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
};

pub fn debug(file_path: &str) -> Result<String, FileSystemOperationError> {
    let mut ft = FileTable::new();
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
