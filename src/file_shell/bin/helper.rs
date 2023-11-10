use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    file_shell::{file_table::file_table::FileTable, root_file::error::FileSystemOperationError},
    SUPER_BLOCK,
};

pub fn get_ft() -> Result<Arc<Mutex<FileTable>>, FileSystemOperationError> {
    // let sb=unsafe{match &SUPER_BLOCK{
    //     Some(x)=>x,
    //     None=>return Err(FileSystemOperationError::InitError(format!("文件系统初始化失败"))),
    // }};
    // let ft=match sb.lock().get_file_table(){
    //     Some(x)=>x,
    //     None=>return Err(FileSystemOperationError::InitError(format!("文件系统初始化失败"))),
    // };
    // Ok(ft)
    todo!()
}

// pub fn get_ft_guard()->Result<Box<MutexGuard<'static,FileTable>>, FileSystemOperationError>{
//     let sb=unsafe{match &SUPER_BLOCK{
//         Some(x)=>x,
//         None=>return Err(FileSystemOperationError::InitError(format!("文件系统初始化失败"))),
//     }};
//     let ftt=match sb.get_file_table(){
//         Some(x)=>x,
//         None=>return Err(FileSystemOperationError::InitError(format!("文件系统初始化失败"))),
//     };
//     let ft=ftt.lock();

//     let mut ft=match ft{
//         Ok(x)=>x,
//         Err(_)=>return Err(FileSystemOperationError::LockError(format!("获取file_table锁时出错"))),
//     };
//     Ok(Box::new(ft))
// }
type FtResult = Result<
    std::sync::MutexGuard<'static, FileTable>,
    std::sync::PoisonError<std::sync::MutexGuard<'static, FileTable>>,
>;
pub fn ft_unwrap<'a>(
    r: Result<
        std::sync::MutexGuard<'a, FileTable>,
        std::sync::PoisonError<std::sync::MutexGuard<'a, FileTable>>,
    >,
) -> Result<std::sync::MutexGuard<'a, FileTable>, FileSystemOperationError> {
    match r {
        Ok(a) => Ok(a),
        Err(_) => Err(FileSystemOperationError::InitError(format!(
            "文件系统初始化失败"
        ))),
    }
}
