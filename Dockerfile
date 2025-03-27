# Usar a imagem Alpine como base
FROM alpine:3.18

# Define o diretório de trabalho
WORKDIR /app

# Instalar dependências necessárias, incluindo ffmpeg e wget
RUN apk update && \
    apk add --no-cache \
    libgcc \
    libc6-compat \
    curl \
    python3 \
    python3-dev \
    ffmpeg \
    bash \
    build-base && \
    rm -rf /var/cache/apk/*

# Instalar o yt-dlp (baixando a versão mais recente diretamente do GitHub)
RUN wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -O /usr/local/bin/yt-dlp && \
    chmod a+rx /usr/local/bin/yt-dlp

# Copiar os arquivos do TubeDrop para dentro do container
COPY target/x86_64-unknown-linux-musl/release/ /app

# Criar um usuário não-root
RUN adduser -D appuser

# Garantir permissões corretas para o diretório /app
RUN chown -R appuser:appuser /app

# Alternar para o usuário não-root
USER appuser

# Expor a porta que o app irá rodar
EXPOSE 3000

# Definir o volume para os downloads
VOLUME /app/downloads

# Executar o aplicativo
CMD ["/app/tubedrop"]
