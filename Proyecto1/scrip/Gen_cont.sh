#!/bin/bash

# Definir nombres de imágenes
IMAGES=("js_high_image" "py_high_image" "js_low_image" "py_low_image")


random_name() {
  cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 10 | head -n 1
}

# Generar 10 contenedores aleatoriamente
for i in $(seq 1 10); do
  # Seleccionar una imagen aleatoria
  IMAGE=${IMAGES[$RANDOM % ${#IMAGES[@]}]}
  
  # Generar un nombre de contenedor aleatorio
  CONTAINER_NAME=$(random_name)
  
  # Ejecutar el contenedor
  docker run -d --name "$CONTAINER_NAME" "$IMAGE"
  
  echo "Contenedor $CONTAINER_NAME creado usando la imagen $IMAGE"
done
