#!/bin/bash
echo "=== Iniciando build ==="
echo "Diretório atual: $(pwd)"

cargo build --release

echo "Conteúdo de src/:"
ls -l src/

echo "Copiando arquivos..."
mkdir -p target/release/src
cp -v -r src/* target/release/src/

echo "Conteúdo de target/release/:"
ls -l target/release/

echo "=== Build completo ==="