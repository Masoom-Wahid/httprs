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
