pub enum file_type{
    File,
    Dir,
}

pub trait raw_file{
    fn get_file_type(&self)->Result<file_type,()>;

}