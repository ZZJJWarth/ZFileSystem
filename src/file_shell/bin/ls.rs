use std::ops::{Deref, DerefMut};

use crate::file_shell::{file_table::file_table::FileTable, root_file::root_file::VFile};

pub fn ls(path: &str) -> String {
    // todo!()
    let mut ft = FileTable::new();
    let ls_dir = ft.open(path).unwrap();

    let dir = ls_dir.as_ref().read();
    let mut dir = match dir {
        Ok(dir) => dir,
        Err(_) => {
            return format!("在获取文件锁时出现错误");
        }
    };
    let dir = dir.deref();
    // println!("{:?}",dir);
    let ans = match dir {
        VFile::ZDir(ref zdir) => zdir.ls(),
        VFile::ZFile(ref file) => {
            format!("输入的路径指向一个文件，但是ls指令应该是文件！")
        }
    };
    ans
}

#[cfg(test)]
#[test]
fn test_ls() {
    let ans = ls("/warth1");
    println!("{}", ans);
}
