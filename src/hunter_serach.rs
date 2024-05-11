extern crate serde;
extern crate serde_json;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs,io::Write,io};
use base64::encode;
use regex::Regex;
#[derive(Serialize, Deserialize)]
struct UrlEntry {
    url: String,
}

#[derive(Serialize, Deserialize, )]
struct Data {
    account_type: String,
    total: u64,
    time: u64,
    arr: Vec<UrlEntry>,

    consume_quota: String,
    rest_quota: String,
    syntax_prompt: String,
}

#[derive(Serialize, Deserialize,)]
struct Response {
    code: u16,
    data: Data,
    message: String,
}

fn get_list(res:&str) -> Vec<String> {
    let data = res;
    let mut list: Vec<String> = vec![];
    let res: Response = serde_json::from_str(data).unwrap();
    // println!("Response Code: {}", res.code);
    for entry in res.data.arr {
        println!("{}", entry.url);
        list.push(entry.url.replace("\"",""));
    }
    // println!("{:?}", list);
    // Ok::<(), E>(());
    return list;
}



#[tokio::main]
async fn main() {
    //读取配置文件
    let file_content = fs::read_to_string("ini/config.ini").expect("无法读取文件");
    let mut key="";
    //正则匹配鹰图key
    let re = Regex::new(r"HunterKey=(.*)").unwrap();
    //循环正则匹配
    for cap in re.captures_iter(&file_content) {
        if let Some(value) = cap.get(1) {
            key= value.as_str();
            println!("当前鹰图APIKey为: {}", key);
        }
    }
    //构造搜索语句
    print!("请输入搜索的字符串: ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // 读取用户输入

    println!("您输入的是: {}", encode(&input)); // 打印用户输入（去掉了结尾的换行符）
    let res=Client::new();
    //Url处理
    let url=format!("https://hunter.qianxin.com/openApi/search?api-key={}&search=MQ==&page=1&page_size=10&is_web=1",key);
    //构造请求
    let lists=res.get(url)
        .send()
        .await.unwrap();
    //获取响应体
    let resp=lists.text().await.unwrap();
    //提取url
    let list= get_list(&resp);
    println!("{:?}",list);
    //输出list
    for url in list {
        println!("{:?}",url);
    }

