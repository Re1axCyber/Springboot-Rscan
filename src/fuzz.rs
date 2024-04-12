use tokio_stream::Stream;
use std::fs::File;
use std::io::{BufRead, BufReader};
use futures::stream::{self, StreamExt};
use reqwest::{Client};
use select::document::Document;
use select::predicate::Name;
use colored::Color::{BrightYellow};
use colored::Colorize;
use html5ever::tendril::fmt::Slice;
use rand::prelude::IndexedRandom;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, COOKIE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use rand::{Rng, thread_rng};
use regex::Regex;

// 定义一个Fuzz结构体来存储配置信息
pub struct Fuzz {
    url: String,// URL地址
    wordlist: String,// 字典文件路径
    proxy: String,// 代理地址
    // client: Client,// 创建一个HTTP客户端
    cookies: String,// Cookie
    sem: String
}
impl Fuzz {
    // 初始化Fuzz对象
    pub fn new(url: &str, wordlist: &str,proxy: &str,cookies: &str,sem: &str) -> Self {
        Fuzz {

            url: url.to_string(),// URL地址
            wordlist: wordlist.to_string(),// 字典文件路径
            // client: Client::new(),// 创建一个HTTP客户端
            proxy: proxy.parse().unwrap(),// 代理地址
            cookies: cookies.to_string(),// Cookie
            sem: sem.to_string(),
        }
    }

    // 从本地文件读取字典
    fn read_wordlist(&self) -> impl Stream<Item = String> {
        // 打开字典文件
        let file = File::open(&self.wordlist).expect("无法打开字典文件");
        // 创建BufReader对象
        let reader = BufReader::new(file);
        // 逐行读取字典
        let lines = reader.lines().flatten();
        // 返回Stream对象
        stream::iter(lines)
    }

    pub fn read_urls(file_path: &str) -> Vec<String> {
        // 读取URL文件
        let file = File::open(file_path).expect("无法打开URL文件");
        // 创建BufReader对象
        let reader = BufReader::new(file);
        // 逐行读取URL
        let urls = reader.lines().flatten().collect();
        urls
    }

    // 获取HTML页面标题
    fn get_title(html: &str, status: reqwest::StatusCode) -> String {
        // 如果响应状态码不是200 OK，则直接返回None
        if status != reqwest::StatusCode::OK {
            return "None".to_string();
        }
        // 解析HTML页面
        let document = Document::from(html);
        // 获取标题元素
        let title_element = document.find(Name("title")).next();
        // 获取标题文本
        match title_element {
            Some(title) => title.text(), // 返回标题文本
            None => "No title found".to_string(),// 如果没有标题元素，则返回"No title found"
        }

    }



    // 执行Fuzz测试
    pub async fn check(&self) {
        let p=format!("{}",self.proxy);
        // let x=format!("{}","");
        let client = if p.clone().is_empty() {
            Client::builder()
                .danger_accept_invalid_certs(true) // 忽略 SSL 证书验证
                .build()
                .expect("Failed to create client")
        } else {
            let proxy = reqwest::Proxy::all(
                p.clone()
            ).expect("Failed to create proxy");

            Client::builder()
                .proxy(proxy)
                .danger_accept_invalid_certs(true) // 忽略 SSL 证书验证
                .build()
                .expect("Failed to create client")
        };
        println!("当前使用的代理是: {}",p.clone().bright_green());
        let base_url = self.url.clone()+"/favicon.ico"; // 定义基础URL
        let response = client.get(base_url.clone()).send().await.unwrap();// 发送GET请求并获取响应
        let re = Regex::new(r"timestamp").unwrap();// 定义正则表达式
        let re2 = Regex::new(r"Not Found").unwrap();// 定义正则表达式
        let content_type = response.headers().get("Content-Type").unwrap_or(&"".parse().unwrap()).to_str().unwrap().to_string();// 获取响应头中的Content-Type
        let source_code = response.text().await.unwrap();// 获取响应体
        let vc=source_code.as_bytes().to_vec();// 将字节数组转换为Vec<u8>
        let digest = format!("{:?}",md5::compute(vc));// 计算MD5哈希值并格式化为字符串
        // println!("{}",source_code);
// 检查页面源代码是否匹配正则表达式
        if re.is_match(&source_code) && re2.is_match(&source_code) { //响应体匹配报错特征
            println!("匹配成功：符合Sprinboot报错特征");// 打印匹配成功信息

        }
        else if content_type.contains("image/x-icon") && digest=="3c525d9a513fc9806cc66b01420d1dfe" // 响应头匹配图标特征
           {
            println!("匹配成功：符合SpringBoot图标特征");
        }
            else{
                println!("匹配失败：不符合SpringBoot特征")
            }

    }
    pub async fn fuzz(&self) {

        // 定义User-Agent头
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
            "Mozilla/5.0 (X11; Linux x86_64; rv:45.0) Gecko/20100101 Firefox/45.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/601.7.7 (KHTML, like Gecko) Version/9.1.2 Safari/601.7.7",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_1 like Mac OS X) AppleWebKit/603.1.30 (KHTML, like Gecko) Version/10.0 Mobile/14E304 Safari/602.1",
            "Mozilla/5.0 (Linux; Android 7.0; SM-G930V Build/NRD90M) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.125 Mobile Safari/537.36"
        ];
        // 定义Accept头
        const ACCEPTS: &[&str] = &[
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "application/xml,application/xhtml+xml,text/html;q=0.9,text/plain;q=0.8,*/*;q=0.7",
            // 添加更多 Accept 标头
        ];
        // 生成随机IP
        fn  generate_random_ip() -> String {
            let mut rng = thread_rng();
            let ip_parts: Vec<String> = (0..4)
                .map(|_| rng.gen_range(1..255).to_string())
                .collect();
            ip_parts.join(".")
        }
        // 定义Accept-Encoding头
        let ae ="Accept-Encoding: gzip, deflate, br";
        // 定义基础URL
        let base_url = self.url.clone();
        // 读取字典
        let wordlist = self.read_wordlist();
        // 生成随机XFF
        let random_xff = generate_random_ip();
        // 使用map和buffer_unordered来并发执行请求
        let num: usize = self.sem.clone().parse().unwrap();
        wordlist
            .map(|word| {// 对每个路径执行操作
                let p=format!("{}",self.proxy);
                let client = if p.is_empty() {
                    Client::builder()
                        .danger_accept_invalid_certs(true) // 忽略 SSL 证书验证
                        .build()
                        .expect("Failed to create client")
                } else {
                    let socks5_proxy = reqwest::Proxy::all(
                        p
                    ).expect("Failed to create proxy");

                    Client::builder()
                        .proxy(socks5_proxy)
                        .danger_accept_invalid_certs(true) // 忽略 SSL 证书验证
                        .build()
                        .expect("Failed to create client")
                };
                // 创建请求
                let mut headers = HeaderMap::new();
                // 随机选择一个User-Agent
                let random_ua = user_agents.choose(&mut thread_rng()).unwrap();
                // 随机选择一个Accept
                let random_accepts = ACCEPTS.choose(&mut thread_rng()).unwrap();
                if self.cookies != ""{
                    // 设置Cookie
                    headers.insert(COOKIE, HeaderValue::from_bytes(&self.cookies.clone().into_boxed_str().into_boxed_bytes()).unwrap());
                }

                // 设置请求头
                headers.insert(USER_AGENT, HeaderValue::from_static(*random_ua));// 设置User-Agent
                headers.insert(ACCEPT_ENCODING, HeaderValue::from_static(ae));// 设置Accept-Encoding
                headers.insert(ACCEPT, HeaderValue::from_static(*random_accepts));// 设置Accept
                headers.insert(HeaderName::from_static("x-forwarded-for"), HeaderValue::from_str(&random_xff).unwrap());// 设置X-Forwarded-For

                // 构建请求URL
                let url = format!("{}/{}", base_url, word);
                // 构建请求
                async move {
                    // 发送请求
                    client.get(&url).headers(headers).send().await
                }

            })

            .buffer_unordered(num) // 设置并发请求数量
            .for_each(|res| async {// 处理每个请求结果
                // 获取请求结果
                match res {
                    Ok(response) => {
                        // 获取响应状态码和URL
                        let status = response.status();
                        let url = response.url().as_str().to_string();
                        // 获取响应HTML
                        let html = response.text().await.unwrap();
                        // 获取响应标题和长度
                        let title = Self::get_title(&html,status);
                        let length = html.len();
                        // 构建结果字符串
                        let color_result=format!("[{:?}] {} - Status: {:?} - Length: {}", title, url, status, length);
                        // 将状态码转换为i32类型
                        let int_status=status.as_u16() as i32;
                        // 根据状态码使用不同颜色打印结果
                        match int_status {
                            200 => println!("{}",color_result.red()),
                            301 => println!("{}",color_result.green()),
                            _  => println!("{}",color_result.color(BrightYellow)),
                        }



                    }
                    // 处理请求错误
                    Err(err) => {
                        eprintln!("请求错误: {}", err);
                    }
                }
            })
            .await;// 等待所有请求完成
    }
}

