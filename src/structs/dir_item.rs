use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DirItem{
    pub date:String,
    pub file_name:String,
    pub file_dir:String,
    pub is_dir:bool,
    pub file_size:u64,
    pub file_size_unit:String,
}