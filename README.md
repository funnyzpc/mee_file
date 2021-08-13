
## mee_file文件系统

### 写在签名

  由于目前smb系统集成存在各种各样的异常现象(smb系统存在占用及访问问题)，所以灵机一动直接顺手写了一个简易的文件系统，无法对标各种DFS;由于存在缺点,如在生产使用请慎重~

### 基础功能
+ 用户可视化界面，支持浏览器浏览文件
+ 文件上传接口
+ 文件浏览接口(目录有哪些文件 for api)
+ 文件下载接口(使用api浏览目录后获取目录文件以下载)
+ 其它~

### 开始
+ 运行
  `cargo run`
  
+ 用户端
  `http://127.0.0.1:8012/mee_file`

+ api
  - 验证
  `http://127.0.0.1:8012/auth`
  
  - 上传(POST)
  `http://127.0.0.1:8012/upload`
  
  - 目录文件
  `http://127.0.0.1:8012/list_api`
  
  - 下载
  `http://127.0.0.1:8012/download`

+ 打包
  - `cargo build --release`
  - `cd target/release`
  - `mee_file.exe` 即是
  
  - 若加壳(需安装upx): 
  
    `upx --backup --brute mee_file.exe`

### issues
+ 列表样式调整[完成]
+ 上传文件限制(限制256MB)
+ 删除目录及文件(目录下有文件则不可删除)[完成]
+ 创建目录[完成]
+ 文件预览
+ api接口优化(bug修复+代码优化)
+ 文件下载功能(异步实现)
+ 文件分享功能
+ 目录权限功能
+ 多文件上传错误问题
+