# **MANUAL DE TECNICO**

![icono](Img-tecnico/Tecnico.png)

## StatPy Convertor

## **INDICE**
- [**MANUAL DE TECNICO**](#manual-de-tecnico)
  - [StatPy Convertor](#statpy-convertor)
  - [**INDICE**](#indice)
  - [**SOBRE EL PROGRAMA**](#sobre-el-programa)
  - [**CONOCIMIENTOS PREVIOS**](#conocimientos-previos)
  - [**ESPECIFICACIONES TECNICAS**](#especificaciones-tecnicas)
  - [**FUNCIONES DEL CODIGO**](#funciones-del-codigo)

## **SOBRE EL PROGRAMA**
El proyecto tiene como objetivo desarrollar una aplicación que ofrezca dos
funcionalidades principales: El objetivo de este proyecto es aplicar todos los conocimientos adquiridos en la unidad 1, con la
implementación de un gestor de contenedores mediante el uso de scripts, módulos de kernel,
lenguajes de programación y la herramienta para la creación y manejo de contenedores más
popular, Docker. Con la ayuda de este gestor de contenedores se podrá observar de manera más
detallada los recursos y la representación de los contenedores a nivel de procesos de Linux y como
de manera flexible pueden ser creados, destruidos y conectados por otros servicios.

## **CONOCIMIENTOS PREVIOS**
Los conocimientos mínimos que deben tener las personas que operarán el codigo y deberán utilizar este manual son:
- Conocimientos básicos de Rust
- Conocimientos básicos de python
- Conocimientos básicos de docker
- Conocimientos básicos en apis
- Conocimiento básico de scrips
- Conocimiento básico de modulos
- Conocimiento básico cronjob
- Conocimiento básico Json
- Conocimiento básico matlitplot

## **ESPECIFICACIONES TECNICAS**
Se debe cumplir con los siguientes requisitos antes de usar el programa.

- **Sistema operativo:** linux ubuntu
- **Lenguaje de programacion:** rust, python
- **Editor de codigo:** Visual Studio Code, Neatbeans, etc...


## **FUNCIONES DEL CODIGO**
A continuacion se estara detallando los manejos de codigo que se requirieron para el desarrollo de nuestro programa

- ### **CODIGO high containers**
  high image con el lenguaje de javascript
  ``` js
   const process = require('process');

    function cpuIntensiveTask() {
    const max = 5000; // Reducir aún más el número de cálculos
    const workInterval = 50;  // Tiempo de trabajo en ms
    const pauseInterval = 950; // Tiempo de pausa en ms

    function doWork() {
        let start = Date.now();
        let elapsed = 0;

        while (elapsed < workInterval) {
            for (let i = 0; i < max; i++) {
                let list1 = [];
                for (let j = 0; j < 500; j++) {  // Reducir aún más el tamaño de los arreglos
                    list1.push(Math.sqrt(j));
                }
                let list2 = list1.map(num => Math.sin(num));
                let result = list2.reduce((sum, num) => sum + num, 0);
            }
            elapsed = Date.now() - start;
        }

        // Pausa para reducir el uso de CPU
        setTimeout(doWork, pauseInterval);
    }

    doWork();
    }

    function ramIntensiveTask() {
        let bigArray = [];
        setInterval(() => {
            bigArray.push(new Array(30000).fill(Math.random())); // Reducir el tamaño de los arreglos
            process.stdout.write('.');
        }, 1000); // Pausar para evitar el uso continuo de memoria
    }

    function run() {
        cpuIntensiveTask();
        ramIntensiveTask();
    }

    run();

  ```
  high image con el lenguaje de python

  ``` python
    import time
    import math
    import threading

    def cpu_intensive_task():
        # Cálculo intensivo en CPU con carga mínima
        while True:
            for _ in range(500):  # Reducir aún más el número de cálculos
                _ = [math.sqrt(i) for i in range(700)]  # Reducir el tamaño de los cálculos
            time.sleep(1)  # Pausa de 1 segundo para reducir el uso de CPU

    def ram_intensive_task():
        # Carga intensiva en RAM
        big_list = []
        while True:
            big_list.append([0] * 80000)  # Mantener el tamaño de la lista
            time.sleep(1)  # Pausa de 3 segundos para reducir el ritmo de consumo de RAM

    # Ejecutar ambas tareas en paralelo
    if __name__ == "__main__":
        cpu_thread = threading.Thread(target=cpu_intensive_task)
        ram_thread = threading.Thread(target=ram_intensive_task)

        cpu_thread.start()
        ram_thread.start()

        cpu_thread.join()
        ram_thread.join()
                             

  ```

- ### **CODIGO low containers**
   low image con el lenguaje de javascript
  ```js
    const express = require('express');
    const app = express();
    const PORT = process.env.PORT || 3000;

    // Variables globales para almacenar el estado
    let cpuResult = 0;
    let ramLength = 0;

    // Función para consumir CPU de manera menos intensiva
    function consumeCPU() {
        setInterval(() => {
            let result = 0;
            for (let i = 0; i < 1000; i++) {  // Reducción en la cantidad de cálculos
                result += Math.sqrt(i) * Math.sin(i);
            }
            cpuResult = result;  // Actualiza la variable global
        }, 1000);  // Ejecutar cada 1000 ms (1 segundo)
    }

    // Función para consumir RAM de manera menos intensiva
    function consumeRAM() {
        let ramArray = [];
        setInterval(() => {
            for (let i = 0; i < 50; i++) {  // Reducir aún más la cantidad de elementos
                ramArray.push(Math.random());
            }
            ramLength = ramArray.length;  // Actualiza la variable global
        }, 2500);  // Aumentar el intervalo de tiempo para reducir el uso de memoria
    }

    // Ruta principal
    app.get('/', (req, res) => {
        res.send(`Hello, World! CPU: ${cpuResult}, RAM items: ${ramLength}`);
    });

    // Iniciar el servidor
    app.listen(PORT, '0.0.0.0', () => {
        console.log(`Server is running on port ${PORT}`);
        // Ejecutar las funciones de consumo
        consumeCPU();
        consumeRAM();
    });

  ```

  low image con el lenguaje de python
  ```python
    from flask import Flask
    import time
    import threading
    import random
    import math

    app = Flask(__name__)

    # Función para consumir CPU de manera menos intensiva
    def consume_cpu():
        while True:
            result = 0
            for i in range(1000):  # Reducción en la cantidad de cálculos
                result += math.sqrt(i) * math.sin(i)
            time.sleep(0.1)  # Pausa para reducir el uso continuo de CPU

    # Función para consumir RAM de manera menos intensiva
    def consume_ram():
        global ram_list
        ram_list = []
        while True:
            for _ in range(500):  # Reducir aún más la cantidad de elementos añadidos
                ram_list.append(random.random())
            time.sleep(1)  # Aumentar la pausa para reducir el uso continuo de RAM

    @app.route('/')
    def index():
        return 'Hello, World!'

    if __name__ == '__main__':
        # Crear y ejecutar hilos para consumir CPU y RAM
        threading.Thread(target=consume_cpu, daemon=True).start()
        threading.Thread(target=consume_ram, daemon=True).start()
        app.run(host='0.0.0.0', port=5000)

  ```

- ### **CODIGO script**

encargado de crear los contenedores con nombres aleatorios

```bash
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

```

- ### **CODIGO del modulo en C**

```c
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/string.h> 
#include <linux/init.h>
#include <linux/proc_fs.h> 
#include <linux/seq_file.h> 
#include <linux/mm.h> 
#include <linux/sched.h> 
#include <linux/timer.h> 
#include <linux/jiffies.h> 
#include <linux/uaccess.h>
#include <linux/tty.h>
#include <linux/sched/signal.h>
#include <linux/fs.h>        
#include <linux/slab.h>      
#include <linux/sched/mm.h>
#include <linux/binfmts.h>
#include <linux/timekeeping.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Henry Mariano Ambrocio terraza Perez-201906562");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y CPU en JSON");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_201906562"
#define MAX_CMDLINE_LENGTH 256
#define CONTAINER_ID_LENGTH 64

static char *get_process_cmdline(struct task_struct *task) {

    struct mm_struct *mm;
    char *cmdline, *p;
    unsigned long arg_start, arg_end, env_start;
    int i, len;


    cmdline = kmalloc(MAX_CMDLINE_LENGTH, GFP_KERNEL);
    if (!cmdline)
        return NULL;

    mm = get_task_mm(task);
    if (!mm) {
        kfree(cmdline);
        return NULL;
    }

    down_read(&mm->mmap_lock);
    arg_start = mm->arg_start;
    arg_end = mm->arg_end;
    env_start = mm->env_start;
    up_read(&mm->mmap_lock);

    len = arg_end - arg_start;

    if (len > MAX_CMDLINE_LENGTH - 1)
        len = MAX_CMDLINE_LENGTH - 1;

    if (access_process_vm(task, arg_start, cmdline, len, 0) != len) {
        mmput(mm);
        kfree(cmdline);
        return NULL;
    }

    cmdline[len] = '\0';

    p = cmdline;
    for (i = 0; i < len; i++)
        if (p[i] == '\0')
            p[i] = ' ';

    mmput(mm);
    return cmdline;
}


static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si;
    struct task_struct *task;
    unsigned long total_jiffies = jiffies;
    int first_process = 1;
    unsigned long total_ram, libre_ram, usado_ram;

    si_meminfo(&si);
    total_ram = si.totalram << (PAGE_SHIFT - 10);
    libre_ram = si.freeram << (PAGE_SHIFT - 10);
    usado_ram = total_ram - libre_ram;

    seq_printf(m, "{\n");
    seq_printf(m, "  \"Memory\": {\n");
    seq_printf(m, "    \"Total_RAM\": %lu,\n", total_ram);
    seq_printf(m, "    \"RAM_libre\": %lu,\n", libre_ram);
    seq_printf(m, "    \"RAM_usado\": %lu\n", usado_ram);

    seq_printf(m, "  },\n");
    seq_printf(m, "\"Processes\": [\n");

    for_each_process(task) {
        if (strcmp(task->comm, "containerd-shim") == 0) {
            unsigned long vsz = 0;
            unsigned long rss = 0;
            unsigned long totalram = si.totalram * 4;
            unsigned long mem_usage = 0;
            unsigned long cpu_usage = 0;
            char *cmdline = NULL;

            if (task->mm) {
                vsz = task->mm->total_vm << (PAGE_SHIFT - 10);
                rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);
                mem_usage = (rss * 10000) / totalram;
            }

            unsigned long total_time = task->utime + task->stime;
            cpu_usage = (total_time * 10000) / total_jiffies;
            cmdline = get_process_cmdline(task);

            if (!first_process) {
                seq_printf(m, ",\n");
            } else {
                first_process = 0;
            }

            seq_printf(m, "  {\n");
            seq_printf(m, "    \"PID\": %d,\n", task->pid);
            seq_printf(m, "    \"Name\": \"%s\",\n", task->comm);
            seq_printf(m, "    \"Cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "    \"VSZ\": %lu,\n", vsz);
            seq_printf(m, "    \"RSS\": %lu,\n", rss);
            seq_printf(m, "    \"MemoryUsage\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "    \"CPUUsage\": %lu.%02lu\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "  }");


            if (cmdline) {
                kfree(cmdline);
            }
        }
    }

    seq_printf(m, "\n]\n}\n");
    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo_json modulo cargado\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo_json modulo desinstalado\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);

```

- ### **CODIGO rust**

encargado de realizar los ordenes y eliminar contenedores de igual form

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use ureq::{Error, Response};
use serde_json::json;
use serde_json::Value;
use chrono::{Local, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "Processes")]
    processes: Vec<Process>,
    #[serde(rename = "Memory")]
    memory: Memory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "VSZ")]
    vsz: i64,
    #[serde(rename = "RSS")]
    rss: i64,
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Memory {
    #[serde(rename = "Total_RAM")]
    total_ram: i32,
    #[serde(rename = "RAM_libre")]
    ram_libre: i32,
    #[serde(rename = "RAM_usado")]
    ram_usado: i32
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    vsz: i64,
    rss: i64,
    memory_usage: f64,
    cpu_usage: f64,
    date: String,
}
#[derive(Debug, Serialize, Clone)]
struct LogMemory {
    total_ram: i32,
    ram_libre: i32,
    ram_usado: i32,
    date: String,
}

impl Process {
    fn get_container_id(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}

impl Eq for Process {}

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cpu_usage
            .partial_cmp(&self.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                other
                    .memory_usage
                    .partial_cmp(&self.memory_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| {
                other
                    .vsz
                    .partial_cmp(&self.vsz)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| {
                other
                    .rss
                    .partial_cmp(&self.rss)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn kill_container(id: &str) -> std::process::Output {
    let output = Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Matando contenedor con id: {}", id);

    output
}

fn analyzer(system_info: SystemInfo, idexcepcion: String) {
    let _log_proc_list: Vec<LogProcess> = Vec::new();
    let mut processes_list: Vec<Process> = system_info.processes;
    let memory_info = system_info.memory;

    
    println!("Total RAM: {}", memory_info.total_ram);
    println!("Free RAM: {}", memory_info.ram_libre);
    println!("Used RAM: {}", memory_info.ram_usado);
    println!("------------------------------");

    // Aquí puedes definir los IDs de contenedores que quieres ignorar
    let excluded_container_ids: HashSet<String> =
        vec![idexcepcion.to_string()]
            .into_iter()
            .collect(); // Añade los IDs que deseas ignorar

    // Filtra los procesos para excluir aquellos con los IDs en excluded_container_ids
    processes_list.retain(|process| !excluded_container_ids.contains(process.get_container_id()));

    processes_list.sort();

    let (highest_list, lowest_list) = processes_list.split_at(processes_list.len() / 2);

    println!("Bajo consumo");
    for process in lowest_list {
        println!("PID: {}, Name: {}, container ID: {}, VSZ: {}, RSS: {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.vsz, process.rss, process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    println!("Alto consumo");
    for process in highest_list {
        println!("PID: {}, Name: {}, container ID: {}, VSZ: {}, RSS: {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.vsz, process.rss, process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    let log_proc_list_arc = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    if lowest_list.len() > 3 {
        for process in lowest_list.iter().take(lowest_list.len() - 3) {
            let log_proc_list_arc = Arc::clone(&log_proc_list_arc);
            let container_id = process.get_container_id().to_string();

            let now = Local::now();
    
            // Define el formato personalizado
            let custom_format = "%Y-%m-%d %H:%M:%S";

            let log_process = LogProcess {
                pid: process.pid,
                container_id: container_id.clone(),
                vsz: process.vsz,
                rss: process.rss,
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                date: now.format(custom_format).to_string(),
            };

            log_proc_list_arc.lock().unwrap().push(log_process.clone());

            let handle = thread::spawn(move || {
                let _output = kill_container(&container_id);
            });
            handles.push(handle);
        }
    }

    if highest_list.len() > 2 {
        for process in highest_list.iter().skip(2) {
            let log_proc_list_arc = Arc::clone(&log_proc_list_arc);
            let container_id = process.get_container_id().to_string();
            let now = Local::now();
    
            // Define el formato personalizado
            let custom_format = "%Y-%m-%d %H:%M:%S";

            let log_process = LogProcess {
                pid: process.pid,
                container_id: container_id.clone(),
                vsz: process.vsz,
                rss: process.rss,
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                date: now.format(custom_format).to_string(),
            };

            log_proc_list_arc.lock().unwrap().push(log_process.clone());

            let handle = thread::spawn(move || {
                let _output = kill_container(&container_id);
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Contenedores matados");
    let log_proc_list = log_proc_list_arc.lock().unwrap();
    for process in log_proc_list.iter() {
        println!("PID: {}, Name: {}, Container ID: {}, VSZ: {}, RSS: {}, Memory Usage: {}, CPU Usage: {} ", process.pid, process.name, process.container_id, process.vsz, process.rss, process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    


    let mut log_mem_list: Vec<LogMemory> = Vec::new();

    let now = Local::now();
    
    // Define el formato personalizado
    let custom_format = "%Y-%m-%d %H:%M:%S";

    let log_mem = LogMemory {
        total_ram:  memory_info.total_ram,
        ram_libre: memory_info.ram_libre,
        ram_usado: memory_info.ram_usado,
        date: now.format(custom_format).to_string(),
    };

    log_mem_list.push(log_mem);

    println!("{:?}", log_mem_list);

    // Si hay un error, se detendrá el programa usando panic
    if let Err(e) = logMem(log_mem_list.clone()) {
        panic!("Error: {:?}", e);
    }

    // Si hay un error, se detendrá el programa usando panic
    if let Err(e) = logProc(log_proc_list.clone()) {
        panic!("Error: {:?}", e);
    }

}



fn logProc(procce: Vec<LogProcess>) -> Result<(), Error> {
    // URL de prueba (httpbin) para realizar la solicitud POST
    let url = "http://localhost:8000/logsProc";

    // Realizando la solicitud POST
    let proccess = json!(procce).to_string(); // Convertimos a String

    // Hacemos el request POST enviando el JSON como cadena
    let response: Result<Response, Error> = ureq::post(url)
        .set("Content-Type", "application/json") // Añadir cabecera
        .send_string(&proccess); // Enviar el JSON convertido a string

    // Manejo de la respuesta
    match response {
        Ok(resp) => {
            let body = resp.into_string()?; // Obtener el cuerpo de la respuesta como String
            println!("Respuesta recibida: {}", body);
        }
        Err(ureq::Error::Status(code, resp)) => {
            println!("Error con código {}: {}", code, resp.into_string()?);
        }
        Err(e) => {
            println!("Error al realizar el POST: {:?}", e);
        }
    }

    Ok(())
}


fn logMem(procce: Vec<LogMemory>) -> Result<(), Error> {
    // URL de prueba (httpbin) para realizar la solicitud POST
    let url = "http://localhost:8000/logsMem";

    // Realizando la solicitud POST
    let proccess = json!(procce).to_string(); // Convertimos a String

    // Hacemos el request POST enviando el JSON como cadena
    let response: Result<Response, Error> = ureq::post(url)
        .set("Content-Type", "application/json") // Añadir cabecera
        .send_string(&proccess); // Enviar el JSON convertido a string

    // Manejo de la respuesta
    match response {
        Ok(resp) => {
            let body = resp.into_string()?; // Obtener el cuerpo de la respuesta como String
            println!("Respuesta recibida: {}", body);
        }
        Err(ureq::Error::Status(code, resp)) => {
            println!("Error con código {}: {}", code, resp.into_string()?);
        }
        Err(e) => {
            println!("Error al realizar el POST: {:?}", e);
        }
    }

    Ok(())
}






fn read_proc_file(file_name: &str) -> io::Result<String> {
    let path = Path::new("/proc").join(file_name);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    let system_info: SystemInfo = serde_json::from_str(json_str)?;
    Ok(system_info)
}

fn run_dockercompose() -> (std::process::Output, String) {
    let file_path = "/home/henryterraza/Documentos/Universidad/Sopes_1/2S-2024/Laboratorio/-SO1_2S2024_201906562/Proyecto1/python_service/docker-compose.yml";

    let output = Command::new("docker-compose")
        .arg("-f")
        .arg(file_path)
        .arg("up")
        .arg("-d")
        .output()
        .expect("failed to execute process");

    println!("servidor de logs levantado con exito!");

    let container_id_output = Command::new("sh")
        .arg("-c")
        .arg("docker ps -a --no-trunc --format \"{{.ID}} {{.Names}}\" | grep python_container | awk '{print $1}'")
        .output()
        .expect("failed to execute process");

    let container_id = String::from_utf8_lossy(&container_id_output.stdout)
        .trim()
        .to_string();

    println!("Contenedor excluido con ID: {}", container_id);
    println!("------------------------------");

    (output, container_id)
}


fn fin_cronjob() {
    let _output = Command::new("sh")
        .arg("-c")
        .arg("crontab -r")
        .output()
        .expect("failed to execute process");

    println!("\ncronjob eliminado!");
}

fn main() {

    // Inicar el archivo yml
    let (_output, container_id) = run_dockercompose();

    let stop_signal = Arc::new(Mutex::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);

    // Configurar el manejador de señales Ctrl+C
    ctrlc::set_handler(move || {
        let mut stop = stop_signal_clone.lock().unwrap();
        *stop = true;
    })
    .expect("Error setting Ctrl+C handler");

    loop {
        if *stop_signal.lock().unwrap() {
            fin_cronjob();
            break;
        }

        let system_info: Result<SystemInfo, _>;
        let json_str = read_proc_file("sysinfo_201906562").unwrap();
        system_info = parse_proc_to_struct(&json_str);

        match system_info {
            Ok(info) => {
                analyzer(info, container_id.clone());
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

```