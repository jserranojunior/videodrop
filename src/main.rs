use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::process::{Command, Output};

#[derive(Deserialize, Serialize)]
struct DataVideo {
    video: String,
    tipo: String,
    formato: String,
}

fn download_youtube(data: DataVideo) {
    let output: Output;
    let video = data.video; // Corrigido para usar a variável video
    if data.tipo == "playlist" {
        if data.formato == "mp3" {
            // Baixar todos os vídeos da playlist e extrair áudio MP3
            output = Command::new("yt-dlp")
                .arg("-x") // Extrair áudio
                .arg("--audio-format")
                .arg("mp3") // Formato MP3
                .arg("--yes-playlist") // Baixar toda a playlist
                .arg(video) // URL da playlist
                .output()
                .expect("Falha ao executar o comando de playlist para áudio");
        } else {
            // Baixar todos os vídeos da playlist no formato vídeo+áudio
            output = Command::new("yt-dlp")
                .arg("-f")
                .arg("bv*+ba/b") // Baixa o melhor vídeo e áudio
                .arg("--yes-playlist") // Baixar toda a playlist
                .arg(video) // URL da playlist
                .output()
                .expect("Falha ao executar o comando de playlist para vídeo");
        }
    } else if data.formato == "mp3" {
        // Baixar apenas o áudio (MP3) de um vídeo único
        output = Command::new("yt-dlp")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3") // Formato MP3
            .arg(video) // URL do vídeo
            .output()
            .expect("Falha ao executar o comando para áudio");
    } else {
        // Baixar vídeo normal (vídeo+áudio)
        output = Command::new("yt-dlp")
            .arg("-f")
            .arg("bv*+ba/b") // Melhor qualidade de vídeo e áudio
            .arg(video) // URL do vídeo
            .output()
            .expect("Falha ao executar o comando de vídeo");
    }

    // Verifica o resultado da execução do comando
    if output.status.success() {
        println!("Saída: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Erro: {}", String::from_utf8_lossy(&output.stderr));
    }
}

async fn handle_data(Json(data): Json<DataVideo>) {
    download_youtube(data);
}

#[tokio::main]
async fn main() {
    // Construção da aplicação com uma rota
    let app = Router::new().route("/submit", post(handle_data));

    // Definir o endereço do servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Falha ao vincular ao endereço");

    println!("Servidor rodando em http://localhost:3000");

    // Inicia o servidor usando axum::serve
    axum::serve(listener, app)
        .await
        .expect("Falha ao iniciar o servidor");
}
