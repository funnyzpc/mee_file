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

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultBuild<T> {
    pub status: i32,
    pub timestamp: i64,
    // 可选参数
    pub msg: String,
    pub data_list: T,
}

impl<T>  ResultBuild<T>{
    #[allow(dead_code)]
    // pub fn success() -> Self {
    pub fn success() -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:String::from("success"),data_list:None}
    }
    #[allow(dead_code)]
    pub fn success_with_msg(msg:String) -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:msg,data_list:None}
    }

    #[allow(dead_code)]
    // pub fn success_with_data(data_list:Vec<T>) -> Self {
    pub fn success_with_data(data_list:Vec<T>) -> ResultBuild<Vec<T>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:String::from("success"),data_list:data_list}
    }

    #[allow(dead_code)]
    // pub fn fail() -> Self {
    pub fn fail() -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:0,timestamp:seconds,msg:String::from("fail"),data_list:None}
    }
    #[allow(dead_code)]
    pub fn fail_with_msg(msg:&str) -> ResultBuild<Option<i8>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:0,timestamp:seconds,msg:msg.to_string(),data_list:None}
    }

}