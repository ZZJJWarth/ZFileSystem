#[derive(Debug)]
pub struct AccessKey{
    pub u_id:u8,
    pub access_file_name:String,
    pub parent_path:String,
}

impl AccessKey{
    pub fn new(u_id:u8,access_file_name:String,parent_path:String)->Self{
        Self{
            u_id,
            access_file_name,
            parent_path,
        }
    }
}