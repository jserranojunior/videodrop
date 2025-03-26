use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::string::String;
use tokio::task;
use tower_http::services::ServeDir;

#[derive(Deserialize, Serialize)]
struct DataVideo {
    video: String,
    tipo: String,
    formato: String,
}

#[derive(Serialize)]
struct Response {
    data: bool,
}

async fn download_youtube(data: DataVideo) -> Response {
    let video = data.video.clone();
    let tipo = data.tipo.clone();
    let formato = data.formato.clone();

    let result = task::spawn_blocking(move || {
        let mut cmd = Command::new("yt-dlp");

        if tipo == "playlist" {
            if formato == "mp3" {
                cmd.args(&["-x", "--audio-format", "mp3", "--yes-playlist"]);
            } else {
                cmd.args(&["-f", "bv*+ba/b", "--yes-playlist"]);
            }
        } else if formato == "mp3" {
            cmd.args(&["-x", "--audio-format", "mp3"]);
        } else {
            cmd.args(&["-f", "bv*+ba/b"]);
        }

        cmd.arg(video);
        let output = cmd.output().expect("Falha ao executar o yt-dlp");

        output.status.success()
    })
    .await
    .unwrap_or(false);

    Response { data: result }
}

async fn handle_data(Json(data): Json<DataVideo>) -> Json<Response> {
    let result = download_youtube(data).await;

    // Return the result wrapped in Json
    Json(result)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/submit", post(handle_data))
        .fallback_service(ServeDir::new("src"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Falha ao vincular ao endereço");

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Falha ao iniciar o servidor");
}
