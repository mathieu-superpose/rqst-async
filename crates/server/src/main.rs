use miniserve::{http, Content, Request, Response};

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

async fn chat(req: Request) -> Response {
    match req {
        Request::Post(body) => {
            // parse the body as JSON
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&body);
            if parsed.is_err() {
                return Err(http::StatusCode::BAD_REQUEST);
            }
            // extract the messages from the JSON
            let parsed = parsed.unwrap();

            let messages = parsed["messages"].as_array().cloned();

            match messages {
                Some(mut messages) => {
                    messages.push("And how does that make you feel?".into());
                    let json = serde_json::json!({ "messages": messages });
                    Ok(Content::Json(json.to_string()))
                }
                None => Err(http::StatusCode::BAD_REQUEST),
            }
        }
        _ => Err(http::StatusCode::METHOD_NOT_ALLOWED),
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await;
}
