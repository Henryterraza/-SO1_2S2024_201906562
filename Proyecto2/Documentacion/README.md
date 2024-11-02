# **MANUAL DE TECNICO  (eStudent)**

![Icono](img_tecnico/Tecnico.png)


## Integrantes

| Carnet     | Nombre                                     |
|------------|--------------------------------------------|
| 201906562  | Henry Mariano Ambrocio Terraza Perez       |

## **INDICE**

- [**OBJETIVOS DEL PROYECTO**](#objetivos-del-proyecto)
- [**DESCRIPCION DEL PROYECTO**](#descripcion-del-proyecto)
- [**ARQUITECTURA IMPLEMENTADA DEL PROYECTO**](#arquitectura-implementada-del-proyecto)
- [**PRESUPUESTO DEL PROYECTO**](#presupuesto-del-proyecto)
- [**INVESTIGACION DE LOS SERVICIOS UTILIZADOS**](#investigacion-de-los-servicios-utilizados)
- [**BIBLIOGRAFIA**](#bibliografia)




### OBJETIVOS DEL PROYECTO

- Administrar una arquitectura en la nube utilizando Kubernetes en Google Cloud Platform (GCP).
- Utilizar el lenguaje de programación Golang, maximizando su concurrencia y aprovechando sus librerías.
- Crear y desplegar contenedores en un Container Registry.
- Entender el funcionamiento de un message broker utilizando Kafka.

### DESCRIPCION DEL PROYECTO

Este proyecto tiene como propósito aplicar los conocimientos adquiridos en las unidades 1 y 2 mediante la implementación de una arquitectura en Google Cloud Platform (GCP) utilizando Google Kubernetes Engine (GKE). El sistema monitorizará las Olimpiadas de la Universidad de San Carlos de Guatemala. A través de Kubernetes y contenedores, se deberá desplegar una arquitectura en la nube capaz de soportar grandes volúmenes de tráfico generado por los participantes de las facultades de Ingeniería y Agronomía, que competirán en Natación, Boxeo y Atletismo. El sistema mostrará en tiempo real las medallas obtenidas por cada facultad utilizando Grafana para el análisis de datos

### STRUCTURA DEPLOYMENT
```yaml
    apiVersion: apps/v1
    kind: Deployment
    metadata:
    name: rust-ing-deploy
    namespace: default
    spec:
    replicas: 1
    selector:
        matchLabels:
        app: rust-ing
    template:
        metadata:
        labels:
            app:  rust-ing
        spec:
        containers:
        - name: rust-ing
            image: hterraza/rust-server:1.3
            ports:
            - containerPort: 8080
            resources:
            requests:
                memory: "128Mi"
                cpu: "100m"
            limits:
                memory: "256Mi"
                cpu: "200m"
```

#### Descripción General

- **apiVersion**: `apps/v1` — Versión de la API de Kubernetes utilizada para los deployments.
- **kind**: `Deployment` — Tipo de recurso en Kubernetes, que asegura que el contenedor tenga el estado deseado.
- **metadata**: 
  - **name**: `rust-ing-deploy` — Nombre del Deployment.
  - **namespace**: `default` — Namespace donde se desplegará este recurso.

#### Especificación (`spec`)

- **replicas**: 1 — Número de réplicas deseadas del contenedor.
- **selector**:
  - **matchLabels**: Define el selector `app: rust-ing` para identificar los pods que pertenecen a este Deployment.

#### Template del Pod (`template`)

Define la configuración de los pods que se crearán:

- **metadata/labels**: Etiqueta `app: rust-ing`, utilizada para asociar los pods con el Deployment.

#### Contenedor (`containers`)

Define el contenedor llamado `rust-ing` con la siguiente configuración:

- **image**: `hterraza/rust-server:1.3` — Imagen del contenedor que se usará.
- **ports**: Expone el puerto `8080` para la aplicación dentro del contenedor.

#### Recursos (`resources`)

Configura los recursos que el contenedor puede utilizar:

- **requests**: Define los recursos mínimos necesarios para iniciar el pod:
  - **memory**: `128Mi` — Memoria mínima requerida.
  - **cpu**: `100m` (milicores) — CPU mínima requerida.
- **limits**: Define los límites de uso del contenedor para evitar un consumo excesivo:
  - **memory**: `256Mi` — Memoria máxima permitida.
  - **cpu**: `200m` — CPU máxima permitida.





### STRUCTURA DE UN SERVICE
```yaml
    apiVersion: v1
        kind: Service
        metadata:
        name: rust-ing-srv
        namespace: default
        spec:
        selector:
            app: rust-ing
        ports:
            - protocol: TCP
            port: 8080
            targetPort: 8080
        type: ClusterIP
```

#### Descripción General

- **apiVersion**: `v1` — Versión de la API de Kubernetes para servicios.
- **kind**: `Service` — Tipo de recurso, utilizado para definir un Service que permite la comunicación entre otros servicios y pods en Kubernetes.
- **metadata**: 
  - **name**: `rust-ing-srv` — Nombre del Service.
  - **namespace**: `default` — Namespace en el que se desplegará este recurso.

#### Especificación (`spec`)

- **selector**:
  - **app**: `rust-ing` — Selector utilizado para asociar este Service con los pods que tienen la etiqueta `app: rust-ing`. Esto garantiza que el tráfico se redirija a los pods que coinciden con el selector.
  
- **ports**: Define los puertos utilizados por el Service para la comunicación:
  - **protocol**: `TCP` — Protocolo de red usado para la comunicación.
  - **port**: `8080` — Puerto en el que el Service escucha internamente en el clúster.
  - **targetPort**: `8080` — Puerto del contenedor que recibe el tráfico redirigido por el Service.

- **type**: `ClusterIP` — Tipo de Service que hace accesible el conjunto de pods solo dentro del clúster de Kubernetes (no es accesible externamente).
