use actix_web::{web, App, HttpServer, Responder};

mod args;
mod etcher;
mod models;
mod run;
mod settings;
mod source;
mod tasks;
mod timer;

// Embed route handler
async fn embed_handler(params: web::Json<models::EmbedParams>) -> impl Responder {
    match tasks::embed::run_embed(params.into_inner()).await {
        Ok(_) => "Embed operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

// Download route handler
async fn download_handler(params: web::Json<models::DownloadParams>) -> impl Responder {
    match tasks::download::run_download(params.into_inner()).await {
        Ok(_) => "Download operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

// Dislodge route handler
async fn dislodge_handler(params: web::Json<models::DislodgeParams>) -> impl Responder {
    match tasks::dislodge::run_dislodge(params.into_inner()).await {
        Ok(_) => "Dislodge operation succeeded".into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

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
