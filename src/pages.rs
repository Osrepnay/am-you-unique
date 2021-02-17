use lazy_static::lazy_static;

fn gen_page(title: &str, body: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang = "en">
            <head>
                <meta charset = "utf8">
                <title>{}</title>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
        title, body,
    )
}

lazy_static! {
    pub static ref INDEX: String = gen_page(
        "Am You Unique",
        r#"
        <p id = "unique"></p>
        <script>
            (async () => {
                let userAgentStr = navigator.userAgent;
                let userAgentJson = JSON.stringify({'user_agent': userAgentStr});
                console.log(userAgentStr);
                let response = await fetch("/add-user-agent", {
                    method: "POST",
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: userAgentJson
                });
                let body = await response.text();
                console.log(body);
                document.getElementById("unique").innerHTML = body;
            })();
        </script>
        "#
    );
}
