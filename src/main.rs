// 引入线程模块
use std::thread;
// 引入tcp server 相关的模块
use std::net::{TcpListener, TcpStream, Shutdown};
// 引入标准库 io 读写模块
use std::io::{Read, Write};


// 处理客户端请求的函数
fn handle_client(mut stream: TcpStream) {
    // 初始化 长度为50 个字符的可修改数组
    let mut data = [0 as u8; 50];
    // 从客户端读取数据到数组，并处理，成功返回 true 使得循环进行下一次读取
    while match stream.read(&mut data){
        // 如果读取成功
        Ok(size) => {

            // 将数据从读取并转换类型
            let tmp =  String::from_utf8_lossy(&data[0..size]);
            
            // 输出一下接收内容和其长度
            println!("Received \n size : {}, content : {}", size, tmp);
            
            // 客户端输入 exit并回车，表示要停止了
            if &*tmp == "exit\r\n"{
                // 返回 false 终止循环
                false
            } else {
                // 否则继续
                // 将收到的内容传输给客户端
                stream.write(&data[0..size]).unwrap();
                //成功返回 true 使得循环进行下一次读取
                true
            }
        },
        // 其他情况表示读取失败
        Err(_) => {
            // 输出一下表示要关闭连接了
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            // 关闭连接
            stream.shutdown(Shutdown::Both).unwrap();
            // 返回 false 终止循环
            false
        }
    }{}

}

// main 函数，程序入口
fn main() {
    // 创建 tcp listener
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // 输出一下，表示服务启动监听在 3333 端口
    println!("Server listening on port 3333");
    // 循环获取客户端请求 socket;
    for stream in listener.incoming() {
        // 用模式匹配进行判断是正常还是错误
        match stream {
            // 获取正常
            Ok(stream) => {
                // 输出一下 客户端地址 表示有客户端接入
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 创建一个线程来处理客户端请求
                thread::spawn(move|| {
                    // 调用处理客户端请求的函数
                    handle_client(stream)
                });
            }
            // 获取异常
            Err(e) => {
                // 输出错误原因
                println!("Error: {}", e);
            }
        }
    }
    // 关闭 tcp 服务
    drop(listener);
}