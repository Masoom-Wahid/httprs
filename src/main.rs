use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;





/*

    TODO: Things to add  ->:
          A URI resoource locator
          image support
          better file handling
          more than 1024 bytes for buffer of the request or even a dynamic allocator based on the buffer
          see if u can write TcpListener uself
          and TcpStrea
          

*/


fn main(){
    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8080";

    let end_point : String =  HOST.to_owned() + ":" + PORT;


    let listener = TcpListener::bind(end_point).unwrap();


    println!("Web server is running on {}",PORT);


    for stream in listener.incoming(){
        let _stream = stream.unwrap();
        println!("connection established");
        handle_connection(_stream);
    }
}


fn handle_connection(mut stream : TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request : {}",String::from_utf8_lossy(&buffer[..]));
    //let response = "HTTP/1.1 200 Ok\r\n\r\n";
    let response_contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}