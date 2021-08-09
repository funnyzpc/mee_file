use serde::{Deserialize, Serialize};

// use chrono::NaiveDateTime;


// #[derive(Serialize, Deserialize)]
// pub struct File {
//     pub name: String,
//     pub time: u64,
//     pub err: String,
// }
//
// #[derive(Debug,Serialize,Deserialize)]
// pub struct Download {
//     pub file_path: String,
// }

#[derive(Debug,Serialize,Deserialize)]
pub struct DownLoadFile {
    pub file_path: Option<String>,
    // 可选参数
    pub file_name: Option<String>,
}