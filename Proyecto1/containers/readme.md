### Comando para prender

```bash
sudo systemctl start docker
```

### Comando para crear imagenes

```bash
sudo docker build -t <name> .

# dentro de la carpeta high
sudo docker build -t js_high_image .
sudo docker build -t py_high_image .
# dentro de la carpeta low
sudo docker build -t js_low_image .
sudo docker build -t py_low_image .
```


### Comandos para crear los contenedores

```bash
sudo docker run  -d --name <name_container> <name_image> 

# dentro de la carpeta high
sudo docker run -d --name high_container_1 js_high_image &&
sudo docker run -d --name high_container_2 py_high_image
# dentro de la carpeta low
sudo docker run -d --name low_container_1 py_low_image &&
sudo docker run -d --name low_container_2 js_low_image 
sudo docker run -d --name low_container_3 low_image
```


### Comando para borrar todo

```bash
sudo docker rm $(sudo docker ps -a -q)

#img
sudo docker rmi $(sudo docker ps -a -q)
```

### iniciar contenedores

```bash
sudo docker start $(sudo docker ps -a -q)
```

### Parar contendores

```bash
sudo docker stop $(sudo docker ps -a -q)
```
