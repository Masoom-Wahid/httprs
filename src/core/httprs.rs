use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;
use std::{env, fs};
use anyhow::Result;
use log::{debug, info};



pub struct HttpRs{
    host : String,
    port : String,
}




impl HttpRs{
    pub fn new(
        host : &str,
        port : &str,
    ) -> Self{


        Self{
            host :host.to_string(),
            port : port.to_string(),
        }
    
    }


    fn get_html_for_dir(&self,dir: std::fs::ReadDir,abs_path : &str) -> String {
        let mut html = String::from("<html><body><h1>Directory Listing</h1><ul>");
    
        for entry in dir {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string();
                    let href_path = abs_path.to_string() + "/" +  &file_name;
                    // Generate a link to each file or directory
                    html.push_str(&format!(
                        "<li><a href=\"{}/\">{}</a></li>",
                        href_path,
                        file_name
                    ));
                }
                Err(e) => {
                    debug!("Error reading directory entry: {:?}", e);
                    // Handle the error (could append an error message to HTML if needed)
                }
            }
        }
    
        html.push_str("</ul></body></html>");
        html
    }
    
    
    fn get_404_page(&self) -> String{
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

    
    fn is_image(&self,path : &str) -> bool{
        let extensions : [&str ; 7] = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "ico"];
        let path = path.to_lowercase();
        for extension in extensions {
            if path.ends_with(extension) {
                return true;
            }
        }
        false
    }


    fn handle_connection(&self,mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
    
        let request = String::from_utf8_lossy(&buffer[..]);
        debug!("Request : {}", request);
    
        let values: Vec<&str> = request.split("\n").collect(); 
        let method_info: Vec<&str> = values[0].split(" ").collect();
        let _method = method_info[0];
        let uri = {
            if method_info.len() == 1{
                "/"
            }else{
                method_info[1]
            }
        };


        // let uri = method_info[1];
        
        
        
        let mut path = match uri {
            "/" => "/index.html",
            _ => uri
            // _ => uri.strip_prefix("/").unwrap_or(uri),
        };
        
        info!("{} => {} {}",stream.local_addr()?,_method,path);



        if path.len() != 1 && path.ends_with("/"){
            path = path.strip_suffix("/").unwrap_or(path);
        }
    
        debug!("Your Path is {path}");
    
        let binding = env::current_dir()?;
        let curr_dir = binding.to_str().unwrap();
    
        let new_request_path : PathBuf = PathBuf::from(&(curr_dir.to_owned() + &path));
        
    
        let this = new_request_path.is_dir();
    
        debug!("Is Dir {this}");
        
        debug!("new_request_path {:?}",new_request_path);
    
        let response = {
            if path != "/"  && new_request_path.is_dir(){
                let data = fs::read_dir(new_request_path.as_path())?;
                let content = self.get_html_for_dir(data,&path);
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
                match fs::read(new_request_path.as_path()) {
                    Ok(content) => {
                        let content_type = {
                            if self.is_image(path){
                                "image/png"
                            }else if path.ends_with(".css") {
                                "text/css"
                            }else{
                                "text/html"
                            }
                        };
                        // let content_type = if self.is_image(path) {
                        //     "image/jpeg"
                        // } else {
                        //     // "application/download"
                        //     "text/html"
                        // };
            
                        let header = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                            content.len(),
                            content_type
                        );
            
                        let mut response = Vec::new();
                        response.extend_from_slice(header.as_bytes());
                        response.extend_from_slice(&content);
                        response
                    },
                    Err(e) => {
                        let not_found_page = self.get_404_page();
                        debug!("{:?}", e);
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


    pub fn serve(self) -> Result<()> {
        let end_point : String =  format!("{}:{}",self.host,self.port);


        let listener = TcpListener::bind(end_point)?;
    
        info!("HttpRs is running on {} and PORT of {}",self.host,self.port);
        
        for stream in listener.incoming(){
            let _stream = stream?;
            self.handle_connection(_stream)?;
        }

        Ok(())
    }
}