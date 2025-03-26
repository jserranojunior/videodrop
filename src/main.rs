use axum::{Json, Router, routing::post};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tokio::task;
use tower_http::services::ServeDir;

#[derive(Deserialize, Serialize)]
struct DataVideo {
    video: String,
    tipo: String,
    formato: String,
    folder: String,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    message: Option<String>,
}

async fn download_youtube(data: DataVideo) -> Response {
    let downloads_dir = Path::new("./downloads");

    let folder_name = if !data.folder.is_empty() {
        data.folder
    } else {
        Utc::now().format("pasta_%Y-%m-%d_%H-%M-%S").to_string()
    };

    let folder_path = downloads_dir.join(&folder_name);

    if let Err(e) = tokio::fs::create_dir_all(&folder_path).await {
        return Response {
            success: false,
            message: Some(format!("Falha ao criar diretório: {}", e)),
        };
    }

    let result = task::spawn_blocking(move || {
        let mut cmd = Command::new("yt-dlp");

        cmd.current_dir(&folder_path);

        if data.tipo == "playlist" {
            if data.formato == "mp3" {
                cmd.args(["-x", "--audio-format", "mp3", "--yes-playlist"]);
            } else {
                cmd.args(["-f", "bv*+ba/b", "--yes-playlist"]);
            }
        } else if data.formato == "mp3" {
            cmd.args(["-x", "--audio-format", "mp3", "--no-playlist"]);
        } else {
            cmd.args(["-f", "bv*+ba/b", "--no-playlist"]);
        }

        cmd.arg(&data.video);

        match cmd.output() {
            Ok(output) if output.status.success() => Response {
                success: true,
                message: None,
            },
            Ok(output) => Response {
                success: false,
                message: Some(String::from_utf8_lossy(&output.stderr).into_owned()),
            },
            Err(e) => Response {
                success: false,
                message: Some(format!("Falha ao executar comando: {}", e)),
            },
        }
    })
    .await
    .unwrap_or_else(|_| Response {
        success: false,
        message: Some("Erro na execução da tarefa".to_string()),
    });

    result
}

async fn handle_data(Json(data): Json<DataVideo>) -> Json<Response> {
    Json(download_youtube(data).await)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/submit", post(handle_data))
        .fallback_service(
            ServeDir::new("src")
                .append_index_html_on_directories(true)
                .precompressed_gzip(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Falha ao vincular ao endereço");

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Falha ao iniciar o servidor");
}
