#!/bin/bash

# Cantidad de contenedores a crear
cant_containers=10

# Contador inicial
counter=1

# Función para generar nombres aleatorios
random_name() {
  cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 10 | head -n 1
}

# Crear contenedores en un bucle
while [ $counter -le $cant_containers ]; do
  container_name=$(random_name)
  docker run -d --name "$container_name" alpine sleep 3600
  echo "Contenedor $container_name creado."
  ((counter++))
done

echo "$cant_containers contenedores creados exitosamente."


