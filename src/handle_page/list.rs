use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::structs::dir_item::DirItem;
use std::fs::DirEntry;
use chrono::{DateTime, Local};
use std::ops::Index;


// 用户文件列表
pub async fn list(hb: web::Data<Handlebars<'_>>,params:web::Query<HashMap<String,String>>) -> HttpResponse {
    // println!("mee_file => list:{:?}",params);
    let  base_dir = std::env::var("BASE_DIR").unwrap();

    // 查找参数目录
    let file_dir = params.get("file_dir");
    if file_dir.is_some() /*&& !file_dir.unwrap().contains("..")*/{
        let  file_dir =  file_dir.unwrap();
        let full_dir_path = format!("{}/{}",base_dir,file_dir);
        let path_object = Path::new(&full_dir_path);
        if path_object.exists() && path_object.is_dir() {
            // 校验目录,防止目录穿越
            let absolute_path = path_object.canonicalize().unwrap().into_os_string().into_string().unwrap();
            let base_dir_path = Path::new(&base_dir).canonicalize().unwrap().into_os_string().into_string().unwrap();
            // println!("path_object:{:?},base_dir:{}",absolute_path,&base_dir_path);// 绝对路径
            if !absolute_path.starts_with(&base_dir_path) || absolute_path.eq(&base_dir_path){
                return list_base_dir(hb,&base_dir).await;
            }
            let mut dir_data_list:Vec<DirItem> = Vec::new();
            for entry in  fs::read_dir(path_object).unwrap(){
                let file_entry:DirEntry = entry.unwrap();
                let date: DateTime<Local> = file_entry.metadata().unwrap().modified().unwrap().into();
                let file_size:u64 = file_entry.metadata().unwrap().len()/1024;
                let dir_item = DirItem{
                    date:date.format("%Y-%m-%d %H:%M:%S").to_string(),
                    file_name:file_entry.file_name().into_string().unwrap(),
                    file_dir:file_dir.to_owned(),
                    is_dir: file_entry.file_type().unwrap().is_dir(),
                    file_size:if file_size>1024{file_size/1024}else{file_size},
                    file_size_unit:if file_size>1024{"MB".to_owned()}else{"KB".to_owned()},
                };
                dir_data_list.push(dir_item);
            }
            let file_dir = absolute_path.index(base_dir_path.len()+1..);
            // println!("file_dir:{},dir_data_list:{:?}",file_dir.replace("\\","/"),dir_data_list);
            let context_path = std::env::var("CONTEXT_PATH").unwrap();
            let data_model = json!({"context_path":context_path,"file_dir":file_dir.replace("\\","/"),"file_list":dir_data_list});
            let result_html = hb.render_template(LIST_HTML,&data_model).unwrap_or(String::from("<p>获取目录失败</p>"));
            // 返回
            return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(result_html);
        }
    }
    return list_base_dir(hb,&base_dir).await;

}

async fn list_base_dir(hb: web::Data<Handlebars<'_>>,base_dir:&String) -> HttpResponse {
    let path_object = Path::new(&base_dir);
    let mut dir_data_list:Vec<DirItem> = Vec::new();
    for entry in  fs::read_dir(path_object).unwrap(){
        let file_entry:DirEntry = entry.unwrap();
        let date: DateTime<Local> = file_entry.metadata().unwrap().modified().unwrap().into();
        let file_size:u64 = file_entry.metadata().unwrap().len()/1024;
        let dir_item = DirItem{
            date:date.format("%Y-%m-%d %H:%M:%S").to_string(),
            file_name:file_entry.file_name().into_string().unwrap(),
            file_dir:".".to_owned(),
            is_dir: file_entry.file_type().unwrap().is_dir(),
            file_size:if file_size>1024{file_size/1024}else{file_size},
            file_size_unit:if file_size>1024{"MB".to_owned()}else{"KB".to_owned()},
        };
        dir_data_list.push(dir_item);
    }
    let context_path = std::env::var("CONTEXT_PATH").unwrap();
    let data_model = json!({"context_path":context_path,"file_dir":".","file_list":dir_data_list});
    let result_html = hb.render_template(LIST_HTML,&data_model).unwrap_or(String::from("<p>获取目录失败</p>"));
    // 返回
    return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(result_html);
}

const  LIST_HTML:&str =
    r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta content="application/xhtml+xml;charset=UTF-8" http-equiv="Content-Type">
        <meta content="no-cache,must-revalidate" http-equiv="Cache-Control">
        <meta content="no-cache" http-equiv="pragma">
        <meta content="0" http-equiv="expires">
        <meta http-equiv="Cache" content="no-cache">
        <meta content="telephone=no, address=no" name="format-detection">
        <meta content="width=device-width, initial-scale=1.0" name="viewport">
        <meta name="apple-mobile-web-app-capable" content="yes" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
        <title>MEE文件中心</title>
    </head>
    <body>
    <div class="main">
        <div class="idx_title">
            <div style="display:inline-block;">
                <span style="font-weight:bold;font-size:24px;margin-right:32px;">目录: {{file_dir}} </span>
                <span style="font-size:16px;" title="上传文件至当前目录">
                    <a href="javascript:void(0);" onclick="select_file();">上传</a>
                    <a href="javascript:void(0);" onclick="create_dir();">创建目录</a>
                </span>
            </div>
        </div>
        <div class="idx_list">
            <div class="list_item list_bold">
                <!-- <div class="list_block">&nbsp;&nbsp;</div> -->
                <div class="list_first" title="试试双击哦~">名称</div>
                <div class="list_second">日期</div>
                <div class="list_third">大小</div>
                <div class="list_block">操作</div>
            </div>
            <div class="list_item">
                <div class="list_first"><a href="{{context_path}}/list?file_dir={{file_dir}}/..">上一级..</a></div>
            </div>
            {{#each file_list}}

            {{#if is_dir}}<!-- 目录 -->
            <div class="list_item" ondblclick="do_sel(this);">
                <!-- <div class="list_block"><input type="checkbox" name="sel" onclick="do_sel(this);"/></div> -->
                <div class="list_first">📁 <a href="{{../context_path}}/list?file_dir={{file_dir}}/{{file_name}}">{{file_name}}</a></div>
                <div class="list_second">{{date}} </div>
                <div class="list_third">{{file_size}} {{file_size_unit}}</div>
                <div class="list_block"><a href="javascript:void(0);" onclick="del(1,'{{file_name}}');">删除</a></div>
            </div>
            {{else}}<!-- 文件 -->
            <div class="list_item" ondblclick="do_sel(this);">
                <!-- <div class="list_block"><input type="checkbox" name="sel" onclick="do_sel(this);"/></div> -->
                <div class="list_first">📄 <a href="{{../context_path}}/download?file_path={{file_dir}}/{{file_name}}" target="_blank">{{file_name}}</a></div>
                <div class="list_second">{{date}} </div>
                <div class="list_third">{{file_size}} {{file_size_unit}}</div>
                <div class="list_block">
                    <a href="javascript:void(0);" onclick="del(0,'{{file_name}}');">删除</a>
                    <a href="javascript:alert('开发中,敬请期待...');">预览</a>
                </div>

            </div>
            {{/if}}

            {{/each}}
        </div>
    </div>

    <!-- 上传文件:start -->
    <div style="display:none;">
        <form method="POST" enctype="multipart/form-data">
            <!-- <input type="text" name="file_dir" class="form-control-file"  value="{{file_dir}}"/> -->
            <input type="file" name="files" class="form-control-file" required="required" multiple onchange="upload(this);"/>
        </form>
    </div>
    <!-- 上传文件:stop -->

    </body>
    <style>
        body{font-size:18px;top:0;left:0;}
        .main{width:84%;margin-left:8%;margin-top:2%;}
        .idx_title{color:#333;}
        .list_block{display: inline-block;}
        .list_bold{font-weight:bold;font-size: 18px;}
        .list_item{margin:4px 8px;}
        .list_first{width:60%;display:inline-block;}
        .list_second{width:15%;display:inline-block;}
        .list_third{width:10%;display:inline-block;}
    </style>
    <script>
        // 上传选择文件
        function select_file(event){
            if(event){
                event.preventDefault();
            }
            document.querySelector("input[name=files]").click();
        }
        // 上传文件
        function upload(dom){
            // check
            // alert("upload=>"+dom.value);
            this.submit(dom.parentElement);
        }

        function submit(form){
            let header = {"enctype":"multipart/form-data","file_dir":encodeURI("{{file_dir}}")};
            // 隐藏对话框
            fetch("upload", {method: 'POST', body: new FormData(form),headers:header})
                .then(response => response.json())
                .then(data =>
                    function () {
                        if(!data || 1!=data.status){
                            alert(data.msg);
                            return;
                        }
                        alert(data.msg);
                        window.location.reload();
                    }()
                )
                .catch(error => alert("上传超时,请刷新后重试:"+error)
                );
        }

        // 条目着色
        function do_sel(dom){
            if(""==dom.style.backgroundColor){
                dom.style.backgroundColor='#ece6ee';
                dom.style.fontWeight='bold';
                return;
            }
            dom.style.backgroundColor="";
            dom.style.fontWeight='normal';
        }

        // 删除
        function del(is_dir,del_path){
            if(!confirm("确定删除吗删除后将不可恢复?"+del_path)){
                return;
            }
            let body_params = new URLSearchParams({"is_dir": is_dir,"del_path":"{{file_dir}}/"+del_path})
            let header = {"Content-Type":"application/x-www-form-urlencoded"};
            fetch("delete",{ method: 'POST', body: body_params ,headers:header})
                .then(response => response.json())
                .then(data =>
                    function () {
                        if(!data || 1!=data.status){
                            alert(data.msg);
                            return;
                        }
                        // alert(data.msg);
                        window.location.reload();
                    }()
                )
                .catch(error => console.log("请求超时,请刷新后重试~")
                );
        }

        // 创建目录
        function create_dir(){
            // input
            let create_dir = prompt("请输入目录名称:");
            if (!create_dir){
                // alert("您取消了输入~");
                return;
            }
            if(create_dir.startsWith(".") || create_dir.indexOf("/")!=-1 || create_dir.indexOf("\\")!=-1 || create_dir.indexOf("..")!=-1 ){
                alert("目录名非法[不可包含.\\/..]");
                return;
            }
            // submit
            let body_params = new URLSearchParams({"file_dir":"{{file_dir}}","create_dir":create_dir})
            fetch("create_dir",{ method:'POST',body: body_params,headers:{"Content-Type":"application/x-www-form-urlencoded"}})
                .then(response => response.json())
                .then(data =>
                    function () {
                        if(!data || 1!=data.status){
                            alert(data.msg);
                            return;
                        }
                        // alert(data.msg);
                        window.location.reload();
                    }()
                )
                .catch(error => console.log("异常,请刷新后重试:"+error)
                );
        }
    </script>
    </html>
    "#;