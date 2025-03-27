# Tubedrop

Tubedrop é uma aplicação para baixar vídeos e músicas de diversas plataformas de streaming e compartilhamento de vídeos, oferecendo suporte para uma ampla gama de sites.

## 🚀 Funcionalidades

- Baixe vídeos e músicas diretamente de sites populares.
- Suporte a múltiplos formatos de áudio e vídeo.
- Interface simplificada para facilitar o uso.
- Armazene seus downloads em um diretório especificado.

## 🛠️ Como Executar com Docker

Para rodar o Tubedrop utilizando Docker, execute o seguinte comando:

```sh
sudo docker run -p 3000:3000 -v /home/jorge/Downloads/docker:/app/downloads tubedrop
```

### 📂 Explicação dos parâmetros:

- `-p 3000:3000` → Mapeia a porta 3000 do contêiner para a porta 3000 do seu sistema.
- `-v /home/jorge/Downloads/docker:/app/downloads` → Define a pasta onde os arquivos baixados serão armazenados.
- `tubedrop` → Nome da imagem Docker do Tubedrop.

## 📥 Sites Suportados

O Tubedrop suporta download de vídeos e músicas de uma ampla variedade de sites, incluindo:

- Plataformas de streaming de vídeo
- Sites de compartilhamento de mídia
- Redes sociais populares

Para obter a lista completa de sites suportados, consulte a [documentação oficial](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md).

## 📌 Requisitos

Para executar o Tubedrop via Docker, certifique-se de ter instalado:

- [Docker](https://docs.docker.com/get-docker/)

## 🛠️ Construindo a Imagem Docker (Opcional)

Caso queira construir a imagem manualmente, execute:

```sh
docker build -t tubedrop .
```

## 📄 Licença

Este projeto é distribuído sob a licença MIT.
