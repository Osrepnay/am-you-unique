use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use woothee::parser::Parser;
mod pages;
mod user_agent;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(pages::INDEX.clone())
}

#[post("/add-user-agent")]
async fn add_user_agent(user_agent: web::Json<user_agent::UserAgent>) -> impl Responder {
    if Parser::new().parse(&user_agent.user_agent).is_none() {
        return HttpResponse::Ok().body("Yes");
    }
    println!("{:?}", user_agent.user_agent);
    let app_id = match std::env::var("BACK4APP_APP_ID") {
        Ok(app_id) => app_id,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("500 Internal Server Error: {}", err))
        }
    };
    let api_key = match std::env::var("BACK4APP_API_KEY") {
        Ok(api_key) => api_key,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("500 Internal Server Error: {}", err))
        }
    };
    let client = actix_web::client::Client::default();
    let user_agent_json = json!({
        "user_agent": user_agent.user_agent
    });
    let mut response_result = client
        .get("https://parseapi.back4app.com/classes/user_agents")
        .header("X-Parse-Application-Id", app_id.clone())
        .header("X-Parse-REST-API-Key", api_key.clone())
        .query(&[("where", user_agent_json.to_string())])
        .unwrap()
        .send()
        .await;
    let response_json = match &mut response_result {
        Ok(response) => {
            response
                .json::<HashMap<String, Vec<HashMap<String, String>>>>()
                .await
        }
        Err(err) => return HttpResponse::BadGateway().body(format!("502 Bad Gateway: {}", err)),
    };
    let response_whole = response_json.unwrap();
    let response_whole = response_whole.get("results");
    let response = match response_whole {
        Some(response) => response,
        None => return HttpResponse::BadGateway().body("502 Bad Gateway: Couldn't parse result"),
    };
    if response.len() == 0 {
        println!("unique");
        let user_agent_json = json!({
            "user_agent": user_agent.user_agent
        });
        let mut response = client
            .post("https://parseapi.back4app.com/classes/user_agents")
            .header("X-Parse-Application-Id", app_id)
            .header("X-Parse-REST-API-Key", api_key)
            .header("Content-Type", "application/json")
            .send_body(&user_agent_json.to_string())
            .await;
        let response_json = match &mut response {
            Ok(response) => response.json::<HashMap<String, String>>().await,
            Err(err) => {
                return HttpResponse::BadGateway().body(format!("502 Bad Gateway: {}", err))
            }
        };
        let response = response_json.unwrap();
        println!("{:?}", response);
        HttpResponse::Ok().body("Yes")
    } else {
        println!("not unique");
        HttpResponse::Ok().body("No")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").expect("PORT Environment Variable not set");
    let mut bind_to = "0.0.0.0:".to_owned();
    bind_to.push_str(port.as_str());
    let store = MemoryStore::new();
    HttpServer::new(move || {
        App::new().wrap(
            RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                .with_interval(Duration::from_secs(60))
                .with_max_requests(30)
        )
        .service(index)
        .service(add_user_agent)
    })
    .bind(bind_to)?
    .run()
    .await
}
