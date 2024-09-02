Nombre: Henry Mariano Ambrocio Terraza Perez
Carnet: 201906562


# TAREA 3
**Descripcion:** Realizar un Servicio en lenguaje RUST el cual servirá para leer un archivo en formato JSON de la carpeta /proc, ejemplo: /proc/sysinfo_2000, el archivo JSON debe tener información de los contenedores que se están ejecutando, identificar contenedores de alto consumo y de bajo consumo y mostrar sus consumos de CPU y RAM.




## sysinfo_201906562.ko creacion
![](./img/Captura%20desde%202024-09-01%2023-28-00.png)

## verificacion del archivo json en /proc/sysinfo_201906562

![](./img/Captura%20desde%202024-09-01%2023-28-13.png)

## Creacion de la imagen de mayor consumo high
![](./img/Captura%20desde%202024-09-01%2023-28-31.png)

## Creacion de la imagen de menor consumo low   
![](./img/Captura%20desde%202024-09-01%2023-28-41.png)

## Verificacion de las imagenes creadas
![](./img/Captura%20desde%202024-09-01%2023-28-53.png)

## Creando los contenedores
![](./img/Captura%20desde%202024-09-01%2023-29-05.png)

## Nuevamente verificacando el contenido del archivo json ya actualizado con los contenedores
![](./img/Captura%20desde%202024-09-01%2023-29-40.png)

## corriendo el archivo rust para mostrar la informacion de los contenedores
![](./img/Captura%20desde%202024-09-01%2023-30-21.png)
