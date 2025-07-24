#!/bin/bash

# Copyright (c) 2025 DrxcoDev2
# MIT License
# Use this file to clean the template 
# and create a new project

error() {
    echo "Error: $1"
    exit 1
}

echo "Clonando el repositorio..."
git clone https://github.com/DrxcoDev2/rust-web-hello-world.git
cd rust-web-hello-world

echo "Eliminando archivos no necesarios..."
rm -rf .git
rm  templates/index.hbs
rm README.md
rm Makefile
rm LICENSE

echo "Limpiando el proyecto..."
cargo clean

echo "Forzando la eliminacion de /target"
if [ -d "target" ]; then
    rm -rf target
    echo "Target eliminado correctamente"
fi

input=""
while [ -z "$input" ]; do
    read -p "Ingrese el nombre del proyecto: " input
    if [ -z "$input" ]; then
        error "El nombre del proyecto no puede estar vacio."
    fi
    cd ..
    mv rust-web-hello-world $input
    echo "Proyecto renombrado correctamente a $input"
    cd $input
done

input2=""
while [ -z "$input2" ]; do
    read -p "Ingrese el nombre del archivo de entorno: " input2
    if [ -z "$input2" ]; then
        error "El nombre del archivo de entorno no puede estar vacio."
    fi
    touch .env
    echo "Archivo de entorno creado correctamente"
done




