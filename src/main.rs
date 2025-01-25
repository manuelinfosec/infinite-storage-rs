use actix_web::{web, App, HttpServer, Responder};

mod args;
mod etcher;
mod run;
mod settings;
mod source;
mod structs;
mod timer;
mod tasks;

// Embed route handler
async fn embed_handler(params: web::Json<structs::EmbedParams>) -> impl Responder {
    match embed::run_embed(params.into_inner()).await {
        Ok(_) => "Embed operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

// Download route handler
async fn download_handler(params: web::Json<structs::DownloadParams>) -> impl Responder {
    match download::run_download(params.into_inner()).await {
        Ok(_) => "Download operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

// Dislodge route handler
async fn dislodge_handler(params: web::Json<structs::DislodgeParams>) -> impl Responder {
    match dislodge::run_dislodge(params.into_inner()).await {
        Ok(_) => "Dislodge operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/embed", web::post().to(embed_handler))
            .route("/download", web::post().to(download_handler))
            .route("/dislodge", web::post().to(dislodge_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
