use crate::core::utils::{get_404_page, get_curr_dir, get_html_for_dir, get_index_html};
use anyhow::Result;
use log::{debug, info};
use mime_guess;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;
use std::{env, fs};

pub struct HttpRs {
    host: String,
    port: String,
    curr_dir: String,
    no_index_html: bool,
}

impl HttpRs {
    pub fn new(host: &str, port: &str, path: &str, no_index_html: bool) -> Self {
        let curr_dir: String = {
            if path == "." {
                get_curr_dir()
            } else {
                path.to_string()
            }
        };

        Self {
            host: host.to_string(),
            port: port.to_string(),
            curr_dir,
            no_index_html,
        }
    }

    fn get_content_type(&self, path: &std::path::Path) -> String {
        match mime_guess::from_path(path).first() {
            Some(p) => p.to_string(),
            None => "text/html".to_string(),
        }
    }

    fn parse_tcp_data(&self, data: &str) -> Vec<String> {
        let tcp_data: Vec<&str> = data.split(" ").collect();
        let mut tcp_res: Vec<String> = Vec::new();

        tcp_res.push(tcp_data[0].to_string());

        if tcp_data.len() == 1 {
            tcp_res.push("/".to_string());
        } else {
            tcp_res.push(tcp_data[1].to_string());
        }

        tcp_res
    }

    fn parse_packet(&self, request: std::borrow::Cow<str>) -> Vec<String> {
        let data: Vec<&str> = request.split("\n").collect();
        //let mut result: Vec<String> = Vec::new();
        // in future i might want to use other data such as
        // host,user-agent,referrer,so for now it is justified to have to functions
        // one for whole parse_packet and one for parse_tcp_packet
        self.parse_tcp_data(data[0])
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request = String::from_utf8_lossy(&buffer[..]);
        debug!("Request : {}", request);

        let tcp_data = self.parse_packet(request);
        let method = &tcp_data[0].as_str();
        let uri = tcp_data[1].as_str(); // let uri = method_info[1];

        let mut path = match uri {
            "/" => "/index.html",
            _ => uri, // _ => uri.strip_prefix("/").unwrap_or(uri),
        };

        info!("{} => {} {}", stream.local_addr()?, method, path);

        if path.len() != 1 && path.ends_with("/") {
            path = path.strip_suffix("/").unwrap_or(path);
        }

        debug!("Your Path is {path}");

        let new_request_path: PathBuf = PathBuf::from(&(self.curr_dir.to_owned() + &path));

        let this = new_request_path.is_dir();

        debug!("Is Dir {this}");

        debug!("new_request_path {:?}", new_request_path);

        let response = {
            if path != "/" && new_request_path.is_dir() {
                let data = fs::read_dir(new_request_path.as_path())?;
                let content = get_html_for_dir(data, &path);
                let content_type = "text/html";

                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                    content.len(),
                    content_type
                );

                let mut dir_response: Vec<u8> = Vec::new();
                dir_response.extend_from_slice(header.as_bytes());
                dir_response.extend_from_slice(&content.as_bytes());
                dir_response
            } else {
                match fs::read(new_request_path.as_path()) {
                    Ok(content) => {
                        let content_type: String =
                            self.get_content_type(new_request_path.as_path());

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
                        if path == "/index.html" {
                            let boiler_page = {
                                if !self.no_index_html {
                                    get_index_html().as_bytes().to_vec()
                                } else {
                                    get_html_for_dir(fs::read_dir(&self.curr_dir)?, "")
                                        .as_bytes()
                                        .to_vec()
                                }
                            };

                            let header = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                                boiler_page.len(),
                                "text/html"
                            );

                            let mut response: Vec<u8> = Vec::new();
                            response.extend_from_slice(header.as_bytes());
                            response.extend_from_slice(&boiler_page);
                            response
                        } else {
                            let not_found_page = get_404_page();
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
            }
        };

        stream.write_all(&response)?;
        stream.flush()?;
        Ok(())
    }

    pub fn serve(self) -> Result<()> {
        let end_point: String = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(end_point)?;

        info!(
            "HttpRs is running on {} and PORT of {}",
            self.host, self.port
        );

        for stream in listener.incoming() {
            let _stream = stream?;
            self.handle_connection(_stream)?;
        }

        Ok(())
    }
}
