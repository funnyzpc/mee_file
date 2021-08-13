package com.mee.util;

import cn.hutool.http.HttpRequest;
import cn.hutool.http.HttpResponse;
import com.mee.common.util.DateUtil;
import com.mee.common.util.HmacSHA512Util;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.net.URLEncoder;

public class MeeFilesTest {
/**
  【注意】 需要在spring项目pom.xml文件中引入hutool依赖
  		<dependency>
  			<groupId>cn.hutool</groupId>
  			<artifactId>hutool-all</artifactId>
  			<version>5.3.10</version>
  		</dependency>
**/
    //  密钥
    private static final String KEY = "faccba02e2d544c7bc1a7b3067d71d73";

    // auth::验证
    @Test
    public void test01(){
        String request_url = "http://127.0.0.1:8012/mee_file/api/auth";
        String timestamp = String.valueOf(System.currentTimeMillis()/1000);
        String from = "default_v1";
        String access_token= HmacSHA512Util.enc(String.format("%s||%s",timestamp,from),KEY);
        String result = HttpRequest.get(request_url)
                .header("from",from)
                .header("Content-Type","application/json")
                .header("timestamp",timestamp)
                .header("access_token",access_token)
                // .body(dataStr).execute().body();
                .execute().body();
        System.out.println(result);
    }

    // send file::上传文件
    @Test
    public void test02()throws Exception{
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
        String request_url = "http://127.0.0.1:8012/mee_file/api/upload";
        String timestamp = String.valueOf(System.currentTimeMillis()/1000);
        final String path = URLEncoder.encode( "我的目录","UTF-8");
        File[] files = new File("C:\\Users\\60003843\\Desktop\\tmp\\rrr").listFiles();
        String from = "default_v1";

        // ===>远程调用
        // String access_token= HmacSHA512Util.enc(String.format("%s||%s",timestamp,from),KEY);
        String access_token= HmacSHA512Util.enc(String.format("%s||%s",timestamp,from),"faccba02e2d544c7bc1a7b3067d71d73");
        String result = HttpRequest.post(request_url)
                .form("files",files)
                .header("Content-Type","application/form-data")
                .header("timestamp",timestamp)
                // 上传到哪个目录
                .header("path",path)
                // 来自
                .header("from",from)
                // token
                .header("access_token",access_token)
                .execute().body();
        System.out.println(result);
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
    }


    // download file::下载文件
    @Test
    public void test03(){
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
        final String file = "我的目录/p4787b4a2fe434631b8f10d4_hd49f72e26b7442bcbaf54d9.png";
        String url = "http://127.0.0.1:8012/mee_file/api/download?file_path="+file;
        String timestamp = String.valueOf(System.currentTimeMillis()/1000);
        String from = "default_v1";
        String access_token= HmacSHA512Util.enc(String.format("%s||%s",timestamp,from),"faccba02e2d544c7bc1a7b3067d71d73");
        HttpResponse response = HttpRequest.get(url)
                .header("timestamp",timestamp)
                .header("from",from)
                .header("access_token",access_token)
                .timeout(60*1000).execute();
        int http_status = response.getStatus();
        if(200 != http_status){
            System.out.println("body:"+response.body());
            return;
        }
        // 下载文件
        String download_file_path= "D:/tmp/"+new File(file).getName();
        if(200 == response.getStatus()){
            response.writeBody(download_file_path);
        }
        System.out.println("下载文件:"+file);
        System.out.println("下载目录:"+download_file_path);
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
    }


    // list file::目录文件
    @Test
    public void test04(){
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
        // final String file_dir = "oneleaf_tb_ad/2021/05/25";
        final String file_dir = "我的目录";
        String request_url = "http://127.0.0.1:8012/mee_file/api/list?file_dir="+file_dir;
        String timestamp = String.valueOf(System.currentTimeMillis()/1000);
        final String from = "default_v1";

        // ===>远程调用
        String access_token= HmacSHA512Util.enc(String.format("%s||%s",timestamp,from),KEY);
        String result = HttpRequest.get(request_url)
                .header("Content-Type","application/json")
                .header("timestamp",timestamp)
                // 来自
                .header("from",from)
                // token
                .header("access_token",access_token)
                .execute().body();
        System.out.println(result);
        System.out.println("===>start:"+ DateUtil.now().format(DateUtil.FORMAT_DAY_TIME));
    }

}
