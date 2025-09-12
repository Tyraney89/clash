use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clash_backend::{handlers, services, Settings};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Clash Royale Stats API",
        "version": "1.0.0",
        "endpoints": {
            "player": "/api/player/{player_tag}",
            "health": "/health"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let settings = Settings::from_env().unwrap_or_else(|_| Settings::default());
    
    println!("Starting Clash Royale Stats API server on http://{}:{}", 
             settings.server.host, settings.server.port);
    
    // Create the Clash API service
    let clash_api_service = services::ClashApiService::new(settings.clash_api);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(clash_api_service.clone()))
            .service(hello)
            .service(handlers::health::health)
            .service(handlers::player::get_player)
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run()
    .await
}
