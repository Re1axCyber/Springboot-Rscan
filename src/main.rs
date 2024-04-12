mod fuzz;
mod Check_Spring;
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]// 定义命令行参数
#[command(author="Caojiang", name = "SpringFuzz", version = "0.1.0", about, long_about = None)]// 设置作者、版本、介绍等
struct Args {
    /// The URL to scrape
    #[clap(short='u', long="url", value_name = "URL",default_value = "")]// 设置url
    url: String,
    /// The Dict to Fuzz
    #[clap(short='d', long="dict", value_name = "dict", default_value = "")]// 参数读取字典路径
    dict: String,
    /// The URL_File to scrape
    #[clap(short='f', long="urls", value_name = "urlfile",default_value = "")]// 参数读取url文件路径
    urlfile: String,
    /// The Proxy to use
    #[clap(short='p', long="proxy", value_name = "proxy",default_value = "")]// 定义代理
    proxy: String,
    /// The Cookies to use
    #[clap(short='c', long="cookies", value_name = "cookies",default_value = "")]// 定义cookie
    cookies: String,
    /// The sem to use
    #[clap(short='s', long="sem", value_name = "sem",default_value = "10")]// 定义cookie
    sem: String,
}

use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();//解析命令行参数
    let url_file = args.urlfile;//获取url文件路径
    let url = &args.url;//获取url
    let logo= r"
 __            _             _                 _          __
/ _\_ __  _ __(_)_ __   __ _| |__   ___   ___ | |_       /__\___  ___ __ _ _ __
\ \| '_ \| '__| | '_ \ / _` | '_ \ / _ \ / _ \| __|____ / \// __|/ __/ _` | '_ \
_\ \ |_) | |  | | | | | (_| | |_) | (_) | (_) | ||_____/ _  \__ \ (_| (_| | | | |
\__/ .__/|_|  |_|_| |_|\__, |_.__/ \___/ \___/ \__|    \/ \_/___/\___\__,_|_| |_|
   |_|                 |___/
                                                                     Author:Re1axCyber.
 ".green();
    println!("{}",logo);
    if  url_file != ""{//如果url文件不为空
        //读取url文件
        let urls = fuzz::Fuzz::read_urls(&url_file);
        //遍历url文件中的url
        for url in urls {
            let fuzzer = fuzz::Fuzz::new(&url, &*args.dict,&*args.proxy,&*args.cookies,&*args.sem);//创建Fuzz对象
            fuzzer.check().await;//调用fuzz方法
            fuzzer.fuzz().await;//调用fuzz方法
        }

    } else if url != ""{//如果url不为空
        //创建Fuzz对象
        let fuzzer = fuzz::Fuzz::new(&url, &*args.dict,&*args.proxy,&*args.cookies,&*args.sem);//创建Fuzz对象
        fuzzer.check().await;//调用fuzz方法
        fuzzer.fuzz().await;//调用fuzz方法
    } else {
        eprintln!("{}", "请提供URL文件或单个URL".bright_yellow());//输出错误信息
    }
}
