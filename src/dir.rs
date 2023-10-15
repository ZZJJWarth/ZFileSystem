mod raw_file;
struct File{
    data_entry:u64,
    f_type:file_type::Dir,
}

impl file_type for File{
    fn get_file_type(&self)->Result<file_type,()>{
        self.f_type
    }
}