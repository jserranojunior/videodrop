# VideoDrop

VideoDrop é uma aplicação para baixar vídeos e músicas de diversas plataformas de streaming.

![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)
![Node.js](https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white)

## 🚀 Funcionalidades

- 📥 Download de vídeos e músicas
- 🎧 Conversão para MP3/MP4
- 🖥️ Interface web amigável
- 📂 Armazenamento local
- 🐳 Container Docker pronto

## 🐳 Instalação via Docker Compose

1. Crie `docker-compose.yml`:

```yaml
version: "3.8"

services:
  VideoDrop:
    image: jserranojunior/VideoDrop
    ports:
      - "3050:3000"
    volumes:
      - ./downloads:/app/downloads
    restart: unless-stopped
```

2. Execute o container:

```bash
docker-compose up -d
```

3. Acesse: http://localhost:3050

## 💻 Comando Docker Direto

```bash
docker run -d \
  -p 3000:3000 \
  -v /caminho/absoluto/downloads:/app/downloads \
  --name VideoDrop \
  jserranojunior/VideoDrop
```

## 🌐 Sites Suportados

- YouTube, Vimeo, SoundCloud, Twitter
- TikTok, Instagram, Twitter, Pinterest
- +100 sites

## 📄 Licença

MIT License - [LICENSE](LICENSE)
