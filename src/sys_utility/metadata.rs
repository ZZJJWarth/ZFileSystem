#[derive(Debug,Clone, Copy)]
pub struct Metadata {
    file_len: u32,
    max_len: u32,
}

impl Metadata {
    pub fn new(file_len: u32, max_len: u32) -> Self {
        Self {
            file_len,
            max_len,
        }
    }

    pub fn get_file_len(&self) -> u32 {
        self.file_len
    }
    
    pub fn set_file_len(&mut self, file_len: u32) {
        self.file_len = file_len;
    }

    pub fn get_max_len(&self) -> u32 {
        self.max_len
    }

    pub fn set_max_len(&mut self, max_len: u32) {
        self.max_len = max_len;
    }
}
