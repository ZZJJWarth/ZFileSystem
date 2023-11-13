use std::collections::HashMap;
pub mod access_key;
pub struct UserManager{
    user_table:HashMap<String,u8>,
}

impl UserManager{
    pub fn new() -> Self{
        let mut table:HashMap<String,u8>=HashMap::new();
        table.insert("Warth".to_string(),1);
        table.insert("SCUT".to_string(),2);
        table.insert("root".to_string(),0);
        Self { user_table: table }
    }

    pub fn find_user(&self,user:&str) -> Option<u8>{
        self.user_table.get(user).cloned()
    }
}