mod fuzz;
mod hunter_serach;
mod zoomeye_search;
use clap::Parser;
use colored::Colorize;
use tokio;

#[derive(Parser)]// 定义命令行参数
#[command(author="Re1ax_Cyber", name = "SpringBoot-Rscan", version = "0.1.0", about=r"
 __            _             _                 _          __
/ _\_ __  _ __(_)_ __   __ _| |__   ___   ___ | |_       /__\___  ___ __ _ _ __
\ \| '_ \| '__| | '_ \ / _` | '_ \ / _ \ / _ \| __|____ / \// __|/ __/ _` | '_ \
_\ \ |_) | |  | | | | | (_| | |_) | (_) | (_) | ||_____/ _  \__ \ (_| (_| | | | |
\__/ .__/|_|  |_|_| |_|\__, |_.__/ \___/ \___/ \__|    \/ \_/___/\___\__,_|_| |_|
   |_|                 |___/
                                                                     Author:Re1ax_Cyber.

Springboot-Rscan——集自由调节并发/代理池/自定义cookie/多url为一身的SpringBoot扫描工具，使用Rust异步并发执行，效率大翻倍！
")]// 设置作者、版本、介绍等
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
    #[clap(short='s', long="sem", value_name = "sem",default_value = "10")]// 定义并发信号量
    sem: String,
    /// The sem to use ，spring or dir
    #[clap(short='m', long="method", value_name = "method",default_value = "spring")]// 定义模式
    method: String,
}


async fn process_url(url: &str) {
    let args = Args::parse();//解析命令行参数
    let fuzzer = fuzz::Fuzz::new(url, &*args.dict, &*args.proxy, &*args.cookies, &*args.sem);
    match args.method.as_str() {
        "spring" => {
            fuzzer.check().await;
            fuzzer.fuzz().await;
        }
        "dir" => fuzzer.fuzz().await,
        _ => {
            println!("{}", "method error".bright_yellow())
        }
    }
}
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
                                                                     Author:Re1ax_Cyber.
 ".green();
    println!("{}",logo);
    if  url_file != ""{//如果url文件不为空
        //读取url文件
        let urls = fuzz::Fuzz::read_urls(&url_file);
        //遍历url文件中的url
        for url in urls {
            //process_url(&*url).await;
            tokio::join!(process_url(&*url));//并发执行
        }


    } else if url != "" {
        // 单独处理URL
        process_url(url).await;
    } else {
        eprintln!("{}", "请提供URL文件或单个URL".bright_yellow());
    }

}