use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use anyhow::Result;




/*

    TODO: Things to add  ->:
          A URI resoource locator
          image support
          better file handling
          more than 1024 bytes for buffer of the request or even a dynamic allocator based on the buffer
          see if u can write TcpListener uself
          and TcpStrea
          

*/


fn main() -> Result<()> {
    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8080";

    let end_point : String =  format!("{}:{}",HOST,PORT);


    let listener = TcpListener::bind(end_point)?;


    println!("Web server is running on {}",PORT);


    for stream in listener.incoming(){
        let _stream = stream?;
        println!("connection established");
        handle_connection(_stream)?;
    }

    Ok(())
}

fn get_404_page() -> String{
    let result : String =  r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>404 Not Found</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                text-align: center;
                background-color: #f0f0f0;
            }
            h1 {
                font-size: 36px;
                margin-top: 0;
            }
        </style>
    </head>
    <body>
        <h1>404 Not Found</h1>
        <p>The page you are looking for does not exist.</p>
    </body>
    </html>
    "#.to_string();

    result 
}


fn is_image(path : &str) -> bool{
    let extensions : [&str ; 7] = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "ico"];
    path.to_lowercase();
    for extension in extensions {
        if path.ends_with(extension) {
            return true;
        }
    }
    false
}


fn get_content_type(file_name: &str) -> String {
    let mut content_type = String::new();

    match file_name.to_lowercase().as_str() {
        "jpg" | "jpeg" => content_type.push_str("image/jpeg"),
        "png" => content_type.push_str("image/png"),
        "gif" => content_type.push_str("image/gif"),
        "bmp" => content_type.push_str("image/bmp"),
        "tiff" => content_type.push_str("image/tiff"),
        "ico" => content_type.push_str("image/ico"),
        _ => content_type.push_str("application/octet-stream"),
    }

    content_type
}

fn get_html_for_dir(dir: std::fs::ReadDir) -> String {
    let mut html = String::from("<html><body><h1>Directory Listing</h1><ul>");

    for entry in dir {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                // Generate a link to each file or directory
                html.push_str(&format!(
                    "<li><a href=\"{}/\">{}</a></li>",
                    file_name,
                    file_name
                ));
            }
            Err(e) => {
                eprintln!("Error reading directory entry: {:?}", e);
                // Handle the error (could append an error message to HTML if needed)
            }
        }
    }

    html.push_str("</ul></body></html>");
    html
}



fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request : {}", request);

    let values: Vec<&str> = request.split("\n").collect(); 
    let method_info: Vec<&str> = values[0].split(" ").collect();
    let method = method_info[0];
    let uri = method_info[1];

    
    
    let path = match uri {
        "/" => "index.html",
        _ => uri.strip_prefix("/").unwrap_or(uri),
    };
    
    if path.ends_with("/"){
        path.strip_suffix("/");
    }

    let meta_data = fs::metadata(&path)?;

    println!("Your path is {}", path);

    println!("it is a dir {}",meta_data.is_dir());

    let response = {
        if meta_data.is_dir(){
            let data = fs::read_dir(path)?;
            let content = get_html_for_dir(data);
            let content_type = "text/html";
    
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                content.len(),
                content_type
            );
    
            let mut dir_response : Vec<u8> = Vec::new();
            dir_response.extend_from_slice(header.as_bytes());
            dir_response.extend_from_slice(&content.as_bytes());
            dir_response 
        }else{
            match fs::read(path) {
                Ok(content) => {
                    let content_type = if is_image(path) {
                        "image/jpeg"
                    } else {
                        "text/html"
                    };
        
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                        content.len(),
                        content_type
                    );
        
                    let mut response = Vec::new();
                    response.extend_from_slice(header.as_bytes());
                    response.extend_from_slice(&content);
                    response
                }
                Err(e) => {
                    let not_found_page = get_404_page();
                    println!("{:?}", e);
                    let header = format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n",
                        not_found_page.len()
                    );
        
                    let mut response = Vec::new();
                    response.extend_from_slice(header.as_bytes());
                    response.extend_from_slice(not_found_page.as_bytes());
                    response
                }
            }
        }
    };
    


    stream.write_all(&response)?;
    stream.flush()?;
    Ok(())
}