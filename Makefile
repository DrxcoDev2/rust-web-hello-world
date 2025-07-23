APP_NAME := rust-web
BIN_PATH := target/debug/$(APP_NAME)

.PHONY: run build exec clean

## Compilar y ejecutar en modo desarrollo
run:
	cargo run

## Compilar el binario
build:
	cargo build 
	chmod +x $(BIN_PATH)

## Ejecutar el binario directamente
exec: build
	@echo "Ejecutando $(BIN_PATH)..."
	./$(BIN_PATH)

## Limpiar la build
clean:
	cargo clean
