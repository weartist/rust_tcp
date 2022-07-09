// 用来对于用户输入原样输入回去
use std::io::prelude::*;
// 用来开启一个tcp监听
use std::net::TcpListener;
// 用来使用收到的用户输入流
use std::net::TcpStream;
// 用来开启一个线程处理接收到的事件，防止多个输入会卡住的情况
use std::thread;
// 用来创建字符串
use std::str;


fn main() {
    // 开启一个监听7878端口的listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // 匹配接收到的流
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程处理（这里没有使用线程池，并不严谨）
                thread::spawn(move|| {
                    // 将处理请求逻辑简单封装
                    handle_connection(stream);
                });
            }
            // 当Result 枚举匹配错误时
            Err(e) => { 
                // 如果链接错误，提示错误信息并停止运行
                panic!("Error is {:?}", e) 
            }
        }
    }
}

// 处理客户端链接事件
/**
* @param stream: TcpStream  客户端传入的流
*/
fn handle_connection(mut stream: TcpStream) {
    // 定义一个存储用的数组，会不断的填充数据进来，需要声明为可变的
    let mut buffer = [0; 1024];
    // 需要持续接收客户端的输入并echo回去，所以需要一个loop
    loop {
        // 读取输入
        let read_size = stream.read(&mut buffer).expect("读取错误！");

        // 如果没输入需要退出该次链接
        if read_size == 0 {
            break;
        }
        // 做一个错误处理，这里只要输入内容不是字符串我们就认为是错误
        match str::from_utf8(&buffer[..read_size]) {
            // 如果转换成功返回字符串值。
            Ok(v) => {
                println!("debug log content is {:?}", v);
                if v.starts_with("exit") {
                    // 输出终止前的消息。
                    stream.write(b"bye.\n").unwrap();
                    break;
                } else {
                // 给客户端显示输入内容
                    stream.write(v.as_bytes()).unwrap();
                }
            },
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(_) => {
                // 给客户端显示遇到错误
                stream.write(b"Error!").unwrap();
                continue;
            },
        };
    }
}
