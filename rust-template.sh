#!/bin/bash

# Copyright (c) 2025 DrxcoDev2
# MIT License
# Use this file to clean the template 
# and create a new project

# COLORS
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

error() {
    echo "${RED}Error: $1${NC}"
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
rm -rf .github/

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
    echo "${GREEN}Archivo de entorno creado correctamente${NC}"
done

input3=""
while [ -z "$input3" ]; do
    read -p "Te gustaria autoelminar este archivo? (y/n):   " input3
    if [ "$input3" = "y" ]; then
        rm rust-template.sh
        echo "${GREEN}Archivo rust-template.sh eliminado correctamente${NC}"
    fi
    if [ "$input3" = "n" ]; then
        echo "${YELLOW}Archivo rust-template.sh no eliminado${NC}"
    fi

done

input4=""
while [ -z "$input4" ]; do
    read -p "Te gustaria inicializar git? (y/n):   " input4
    if [ "$input4" = "y" ]; then
        git init
        echo "${GREEN}Git inicializado correctamente${NC}"
    fi
    if [ "$input4" = "n" ]; then
        echo "${YELLOW}Git no inicializado${NC}"
    fi

    input4a=""
    while [ -z "$input4a" ]; do
        read -p "Ingrese el repositorio remoto: " input4a
        if [ -z "$input4a" ]; then
            error "El repositorio remoto no puede estar vacio."
        fi
        git remote add origin $input4a
        echo "${GREEN}Repositorio remoto agregado correctamente${NC}"
    done

done

    




