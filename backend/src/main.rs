use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_cors::Cors;
use clash_backend::{handlers, repositories, services, Settings};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Clash Royale Stats API",
        "version": "1.0.0",
        "architecture": "Controller → Service → Repository → External API",
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
    
    // Create the repository layer (data access)
    let clash_repository = repositories::ClashApiRepository::new(settings.clash_api);
    
    // Create the service layer (business logic)
    let player_service = services::PlayerService::new(clash_repository);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(player_service.clone()))
            .service(hello)
            .service(handlers::health::health)
            .service(handlers::player::get_player)
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run()
    .await
}
