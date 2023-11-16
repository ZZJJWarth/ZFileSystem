use crate::{
    file_shell::{
        file_table::file_table::FileTable, root_file::error::FileSystemOperationError,
        user::access_key::AccessKey,
    },
    SUPER_BLOCK,
};

use super::helper::{ft_unwrap, get_ft};

pub fn cat(file_path: &str, ackey: AccessKey) -> Result<String, FileSystemOperationError> {
    // let mut ft = FileTable::new();
    let temp = get_ft()?;                           //获取FileTable的锁
    let ft = temp.lock();            
    let mut ft = ft_unwrap(ft)?;                //获取FileTable的锁守卫
    let dir_ptr = ft.open(file_path)?;           //从FileTable中打开获取文件的读写锁
    drop(ft);   //释放FileTable的锁，否则后面可能会用到FileTable，这将导致自身死锁
    let dir_result = dir_ptr.as_ref().read();  
    let dir_guard = match dir_result {      
        Ok(x) => x, //获取到文件的读写锁
        Err(_) => {     //如果获取锁失败就返回错误
            return Err(FileSystemOperationError::LockError(format!(
                "cat:获取文件锁时出错"
            )));
        }
    };
    dir_guard.dir_user_check(ackey)?;   //对文件的访问权限进行检查，如何没有错误就继续执行，如果有错误将返回错误
    dir_guard.file_cat()        //返回文件cat函数产生的字符串
}

