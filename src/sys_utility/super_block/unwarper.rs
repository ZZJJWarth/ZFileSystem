use std::sync::{Arc, Mutex};

use crate::{
    file_shell::root_file::error::FileSystemOperationError,
    sys_utility::bitmap::block_bit_map::BlockBitmap, SUPER_BLOCK,
};

pub fn get_bitmap<'a>() -> Result<Option<Arc<Mutex<BlockBitmap>>>, FileSystemOperationError> {
    let a = unsafe {
        match (&SUPER_BLOCK) {
            Some(e) => e,
            None => return Err(FileSystemOperationError::InitError(format!("系统未初始化"))),
        }
    };
    let guard: std::sync::MutexGuard<'a, super::super_block::SuperBlock> = match a.lock() {
        Ok(s) => s,
        Err(_) => {
            return Err(FileSystemOperationError::LockError(format!("未能获取锁")));
        }
    };

    let bitmap = guard.get_bitmap();
    Ok(bitmap)
}

pub fn unwrap_bitmap<'a>(
    input: &'a Option<Arc<Mutex<BlockBitmap>>>,
) -> Result<
    std::sync::MutexGuard<'a, crate::sys_utility::bitmap::block_bit_map::BlockBitmap>,
    FileSystemOperationError,
> {
    // let bitmap=match guard.get_bitmap(){
    //     Some(x)=>x,
    //     None=>{return Err(FileSystemOperationError::InitError(format!("系统未初始化")))}
    // };

    // let bitmap: std::sync::MutexGuard<'a, crate::sys_utility::bitmap::block_bit_map::BlockBitmap>=match bitmap.lock(){
    //     Ok(s)=>s,
    //     Err(_)=>{return Err(FileSystemOperationError::LockError(format!("未能获取锁")));}
    // };

    // Ok(bitmap)
    let bitmap_ptr = match input {
        Some(s) => s,
        None => {
            return Err(FileSystemOperationError::InitError(format!("初始化未完成")));
        }
    };
    let bitmap = match bitmap_ptr.lock() {
        Ok(l) => l,
        Err(_) => {
            return Err(FileSystemOperationError::InitError(format!("初始化未完成")));
        }
    };
    Ok(bitmap)
    // todo!()
}
