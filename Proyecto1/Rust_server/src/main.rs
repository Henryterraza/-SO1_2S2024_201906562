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

    // println!("{:?}", log_mem_list);

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



fn logs_graficar() -> Result<(), ureq::Error> {
    // URL del endpoint al que queremos hacer el GET
    let url = "http://localhost:8000/Grafics";

    // Realizando la solicitud GET
    let response = ureq::get(url).call();

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
            println!("Error al realizar el GET: {:?}", e);
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
            if let Err(e) = logs_graficar() {
                panic!("Error: {:?}", e);
            }
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
