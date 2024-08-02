use colored::*;
use log::debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

pub fn print_http_rs_ascii() -> () {
    let http_rs: String = r#"
    _    _ _   _         _____
    | |  | | | | |       |  __ \
    | |__| | |_| |_ _ __ | |__) |___
    |  __  | __| __| '_ \|  _  // __|
    | |  | | |_| |_| |_) | | \ \\__ \
    |_|  |_|\__|\__| .__/|_|  \_\___/
                | |
                |_|
    "#
    .to_string();

    let mut colors = [
        Color::TrueColor {
            r: 66,
            g: 64,
            b: 149,
        },
        Color::TrueColor {
            r: 72,
            g: 61,
            b: 139,
        },
        Color::TrueColor {
            r: 123,
            g: 104,
            b: 238,
        },
        Color::TrueColor {
            r: 65,
            g: 105,
            b: 225,
        },
        Color::TrueColor {
            r: 100,
            g: 149,
            b: 237,
        },
        Color::TrueColor { r: 0, g: 0, b: 255 },
        Color::TrueColor {
            r: 30,
            g: 144,
            b: 255,
        },
        Color::TrueColor {
            r: 0,
            g: 191,
            b: 255,
        },
        Color::TrueColor {
            r: 135,
            g: 206,
            b: 250,
        },
        Color::TrueColor {
            r: 173,
            g: 216,
            b: 230,
        },
        Color::TrueColor {
            r: 72,
            g: 61,
            b: 139,
        },
        Color::TrueColor {
            r: 0,
            g: 128,
            b: 128,
        },
        Color::TrueColor {
            r: 0,
            g: 139,
            b: 139,
        },
        Color::TrueColor {
            r: 0,
            g: 255,
            b: 255,
        },
        Color::TrueColor {
            r: 0,
            g: 206,
            b: 209,
        },
        Color::TrueColor {
            r: 32,
            g: 178,
            b: 170,
        },
        Color::TrueColor {
            r: 0,
            g: 250,
            b: 154,
        },
        Color::TrueColor {
            r: 127,
            g: 255,
            b: 212,
        },
        Color::TrueColor {
            r: 72,
            g: 209,
            b: 204,
        },
        Color::TrueColor {
            r: 64,
            g: 224,
            b: 208,
        },
        Color::TrueColor {
            r: 0,
            g: 255,
            b: 127,
        },
        Color::TrueColor {
            r: 50,
            g: 205,
            b: 50,
        },
        Color::TrueColor {
            r: 144,
            g: 238,
            b: 144,
        },
        Color::TrueColor {
            r: 34,
            g: 139,
            b: 34,
        },
        Color::TrueColor { r: 0, g: 255, b: 0 },
        Color::TrueColor { r: 0, g: 128, b: 0 },
        Color::TrueColor {
            r: 85,
            g: 107,
            b: 47,
        },
        Color::TrueColor {
            r: 154,
            g: 205,
            b: 50,
        },
        Color::TrueColor {
            r: 255,
            g: 255,
            b: 0,
        },
        Color::TrueColor {
            r: 255,
            g: 215,
            b: 0,
        },
        Color::TrueColor {
            r: 218,
            g: 165,
            b: 32,
        },
        Color::TrueColor {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::TrueColor {
            r: 255,
            g: 140,
            b: 0,
        },
        Color::TrueColor {
            r: 255,
            g: 69,
            b: 0,
        },
        Color::TrueColor { r: 255, g: 0, b: 0 },
        Color::TrueColor {
            r: 220,
            g: 20,
            b: 60,
        },
        Color::TrueColor {
            r: 178,
            g: 34,
            b: 34,
        },
        Color::TrueColor { r: 139, g: 0, b: 0 },
        Color::TrueColor {
            r: 255,
            g: 20,
            b: 147,
        },
        Color::TrueColor {
            r: 255,
            g: 105,
            b: 180,
        },
        Color::TrueColor {
            r: 255,
            g: 182,
            b: 193,
        },
        Color::TrueColor {
            r: 255,
            g: 192,
            b: 203,
        },
        Color::TrueColor {
            r: 219,
            g: 112,
            b: 147,
        },
        Color::TrueColor {
            r: 199,
            g: 21,
            b: 133,
        },
        Color::TrueColor {
            r: 255,
            g: 0,
            b: 255,
        },
        Color::TrueColor {
            r: 238,
            g: 130,
            b: 238,
        },
        Color::TrueColor {
            r: 221,
            g: 160,
            b: 221,
        },
        Color::TrueColor {
            r: 218,
            g: 112,
            b: 214,
        },
        Color::TrueColor {
            r: 186,
            g: 85,
            b: 211,
        },
        Color::TrueColor {
            r: 153,
            g: 50,
            b: 204,
        },
    ];
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);

    let lines: Vec<&str> = http_rs.lines().collect();
    let choosen_color: &Color = colors.choose(&mut rng).unwrap_or(&colors[0]);
    for (_, line) in lines.iter().enumerate() {
        let colored_line = line.color(*choosen_color);
        println!("{}", colored_line);
    }
}
