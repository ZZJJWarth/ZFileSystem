use crate::file_shell::root_file::error::FileSystemOperationError;

use super::helper::{ft_unwrap, get_ft};

pub fn check() -> Result<String, FileSystemOperationError> {
    let ft = get_ft()?;
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft)?;
    ft.check()
}
