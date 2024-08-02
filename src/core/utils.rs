use log::debug;
use std::env;
pub fn get_404_page() -> String {
    let result: String = r#"
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
        "#
    .to_string();

    result
}

pub fn get_index_html() -> String {
    let result = r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>HttpRs</title>
                    <style>
                        html,
                        body {
                            height: 100%;
                            margin: 0;
                            padding: 0;
                        }
                        .header {
                            padding-top: 36px;
                            width: 500px;
                        }
                        .content {
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            gap: 10px;
                        }

                        .content h1 {
                            color: #ffffff;
                            font-size: 25px;
                        }
                        .get-started {
                            margin-top: 30px;
                            background-color: #5a45fe;
                            color: #ffffff;
                            font-size: 24px;
                            border: none;
                            border-radius: 10px;
                            width: 160px;
                            height: 40px;
                            cursor: pointer;
                        }
                        .get-started:hover {
                            background-color: #f0f0f0;
                            color: #5a45fe;
                        }
                        body {
                            background-image: linear-gradient(
                                to bottom right,
                                #000,
                                #424095
                            );
                            background-repeat: no-repeat;
                            background-size: 100% 100%;
                        }
                    </style>
                </head>
                <body>
                    <main>
                        <div class="header">
                            <h1>HttpRs</h1>
                        </div>
                        <div class="content">
                            <h1>HttpRs , A Minimal Webserver Written In Pure Rust,</h1>
                            <h1>You Are Here Because There Was No index.html</h1>
                            <h1>To Get Started Please Update/Create a index.html file</h1>
                            <button class="get-started">Get Start</button>
                        </div>
                    </main>
                </body>
            </html>
            "#
    .to_string();
    return result;
}

pub fn get_html_for_dir(dir: std::fs::ReadDir, abs_path: &str) -> String {
    let mut html = String::from("<html><body><h1>Directory Listing</h1><ul>");

    for entry in dir {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                let href_path = abs_path.to_string() + "/" + &file_name;
                html.push_str(&format!(
                    "<li><a href=\"{}/\">{}</a></li>",
                    href_path, file_name
                ));
            }
            Err(e) => {
                debug!("Error reading directory entry: {:?}", e);
            }
        }
    }

    html.push_str("</ul></body></html>");
    html
}

pub fn get_curr_dir() -> String {
    let curr_dir_path_buf =
        env::current_dir().expect("Could not read the curr_dir for some reason , god knows why");
    curr_dir_path_buf.to_string_lossy().to_string()
}
