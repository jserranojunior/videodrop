use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    process::Command,
    sync::{Arc, Mutex},
};
use tokio::task;
use tower_http::services::ServeDir;
use uuid::Uuid;

// Estrutura para armazenar o estado do progresso
#[derive(Clone, Serialize)]
struct DownloadProgress {
    status: String, // "waiting", "downloading", "completed", "failed"
    progress: f32,  // 0.0 a 1.0
    message: String,
}

#[derive(Deserialize, Serialize)]
struct DataVideo {
    video: String,
    tipo: String,
    formato: String,
}

#[derive(Serialize)]
struct Response {
    download_id: String,
}

// Definimos um tipo para o estado compartilhado
type SharedState = Arc<Mutex<HashMap<String, DownloadProgress>>>;

async fn download_youtube(data: DataVideo, download_id: String, state: SharedState) {
    let video = data.video;
    let tipo = data.tipo;
    let formato = data.formato;

    // Atualiza o estado para "downloading"
    {
        let mut state = state.lock().unwrap();
        state.insert(
            download_id.clone(),
            DownloadProgress {
                status: "downloading".to_string(),
                progress: 0.0,
                message: "Starting download...".to_string(),
            },
        );
    }

    let result = task::spawn_blocking(move || {
        let mut cmd = Command::new("yt-dlp");

        // Configura os argumentos do comando
        if tipo == "playlist" {
            if formato == "mp3" {
                cmd.args(["-x", "--audio-format", "mp3", "--yes-playlist"]);
            } else {
                cmd.args(["-f", "bv*+ba/b", "--yes-playlist"]);
            }
        } else if formato == "mp3" {
            cmd.args(["-x", "--audio-format", "mp3"]);
        } else {
            cmd.args(["-f", "bv*+ba/b"]);
        }

        cmd.arg(video);

        // Executa o comando
        let output = cmd.output().expect("Failed to execute yt-dlp");

        output.status.success()
    })
    .await
    .unwrap_or(false);

    // Atualiza o estado com o resultado
    let mut state = state.lock().unwrap();
    state.insert(
        download_id,
        DownloadProgress {
            status: if result {
                "completed".to_string()
            } else {
                "failed".to_string()
            },
            progress: 1.0,
            message: if result {
                "Download completed".to_string()
            } else {
                "Download failed".to_string()
            },
        },
    );
}

async fn handle_data(
    State(state): State<SharedState>,
    Json(data): Json<DataVideo>,
) -> Json<Response> {
    let download_id = Uuid::new_v4().to_string();

    // Inicia o download em uma task separada
    tokio::spawn(download_youtube(data, download_id.clone(), state));

    Json(Response { download_id })
}

async fn check_progress(
    Path(download_id): Path<String>,
    State(state): State<SharedState>,
) -> Json<DownloadProgress> {
    let state = state.lock().unwrap();
    let progress = state
        .get(&download_id)
        .cloned()
        .unwrap_or(DownloadProgress {
            status: "not_found".to_string(),
            progress: 0.0,
            message: "Download ID not found".to_string(),
        });

    Json(progress)
}

#[tokio::main]
async fn main() {
    // Cria o estado compartilhado
    let state = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/submit", post(handle_data))
        .route("/progress/{download_id}", get(check_progress)) // Mudei : para {
        .fallback_service(ServeDir::new("src"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
