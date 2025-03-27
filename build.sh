#!/bin/bash
echo "=== Iniciando build ==="
echo "Diretório atual: $(pwd)"

cargo build --release --target x86_64-unknown-linux-musl


echo "Conteúdo de src/:"
ls -l src/

echo "Copiando arquivos..."
mkdir -p target/x86_64-unknown-linux-musl/release/src
cp -v -r src/* target/x86_64-unknown-linux-musl/release/src

echo "Conteúdo de target/release/:"
ls -l target/release/

echo "=== Build completo ==="