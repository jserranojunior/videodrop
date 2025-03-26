# Estágio de construção
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /app

# 1. Baixa e extrai o projeto
RUN wget https://github.com/jserranojunior/tubedrop/archive/refs/tags/v1.0.0.tar.gz -O tubedrop.tar.gz && \
    tar -xzf tubedrop.tar.gz --strip-components=1 && \
    rm tubedrop.tar.gz

# 2. Prepara e executa o build.sh com Cargo
RUN chmod +x build.sh && \
    ./build.sh || { echo "Falha no build.sh"; exit 1; }

# Estágio final com imagem Ubuntu 22.04
FROM ubuntu:22.04

# Instalar dependências e atualizar glibc
RUN apt-get update && \
    apt-get install -y libgcc-s1 wget libc6-dev curl python3 python3-pip && \
    apt-get install -y libc6 && \
    rm -rf /var/lib/apt/lists/*

# Instalar o yt-dlp (baixando a versão mais recente diretamente do GitHub)
RUN wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -O /usr/local/bin/yt-dlp && \
    chmod a+rx /usr/local/bin/yt-dlp

# Instalar ffmpeg
RUN apt-get update && \
    apt-get install -y ffmpeg

# Instalar Rust diretamente no contêiner final
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Garantir que o Cargo esteja no PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Define o diretório de trabalho
WORKDIR /app

# Criar os diretórios necessários para o app e garantir as permissões
RUN mkdir -p /app/target /app/downloads && \
    chmod -R 777 /app /app/target /app/downloads

# 3. Copia os arquivos do estágio de construção para o diretório /app
COPY --from=builder /app /app

# Expõe a porta e define o volume
EXPOSE 3000
VOLUME /app/target/release/downloads

# 4. Executa o comando cargo run
CMD ["cargo", "run"]
