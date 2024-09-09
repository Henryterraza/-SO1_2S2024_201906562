# Dockerfile para un contenedor de alto consumo de CPU y RAM
# Utilizar la imagen base de Node.js
FROM node:16-slim

# Copiar el script JavaScript al contenedor
COPY high.js /high.js

# Establecer el comando por defecto para ejecutar el script
CMD ["node", "/high.js"]
