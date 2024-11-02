from fastapi import FastAPI # type: ignore
import os
import json
from typing import List
from models.models import LogProcess
from models.models import LogMemor


import matplotlib.pyplot as plt
import datetime

app = FastAPI()


@app.get("/")
def read_root(): 
    return {"Hello": "World"}


@app.get("/Grafics")
def read_root():

    grafica_Mem()
    grafica_Proc()

    return {"Hello": "Grafica generada con exito"}




def grafica_Proc():
    logs_file = 'logs/logs_processes.json'
    
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Convert the timestamps into datetime objects
    timestamps = [datetime.datetime.strptime(entry["date"], "%Y-%m-%d %H:%M:%S") for entry in existing_logs]

# Extract memory and CPU usage
    memory_usage = [entry["memory_usage"] for entry in existing_logs]
    cpu_usage = [entry["cpu_usage"] for entry in existing_logs]

    # Plotting the data
    plt.figure(figsize=(10, 6))

    plt.plot(timestamps, memory_usage, label='Memoria usada (%)', marker='o', color='purple')
    plt.plot(timestamps, cpu_usage, label='CPU usado (%)', marker='x', color='green')

    # Formatting the plot
    plt.xlabel('fecha y hora')
    plt.ylabel('Usado (%)')
    plt.title('CPU y Memory usados')
    plt.legend()

    # Rotating the date labels
    plt.xticks(rotation=45)

    plt.tight_layout()
    plt.savefig("./grafica_Proc.png")  # Ruta para guardar el archivo
    plt.close()

def grafica_Mem():
    logs_file = 'logs/logs_memory.json'
    
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

   # Convert the timestamps into datetime objects
    timestamps = [datetime.datetime.strptime(entry["date"], "%Y-%m-%d %H:%M:%S") for entry in existing_logs]

    # Extract memory values
    mem_libre = [entry["ram_libre"] for entry in existing_logs]
    mem_usado = [entry["ram_usado"] for entry in existing_logs]

    # Plotting the data
    plt.figure(figsize=(10, 6))

    plt.fill_between(timestamps, mem_usado, label='Memory Used', color='red', alpha=0.7)
    plt.fill_between(timestamps, mem_usado, [entry["total_ram"] for entry in existing_logs], label='Free Memory', color='blue', alpha=0.5)

    # Formatting the plot
    plt.xlabel('Timestamp')
    plt.ylabel('Memory (MB)')
    plt.title('Memory Usage Over Time')
    plt.legend()

    # Rotating the date labels
    plt.xticks(rotation=45)

    plt.tight_layout()
    plt.savefig("./grafica_Mem.png")  # Ruta para guardar el archivo
    plt.close()
    


@app.post("/logsProc")
def get_logs(logs_proc: List[LogProcess]):
    logs_file = 'logs/logs_processes.json'
    
    # Checamos si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Agregamos los nuevos logs a la lista existente
    new_logs = [log.dict() for log in logs_proc]
    existing_logs.extend(new_logs)

    # Escribimos la lista de logs en el archivo logs.json
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"status": True, "msj": "Logs de procesos guardado con exito"}
    

@app.post("/logsMem")
def get_logs(logs_proc: List[LogMemor]):
    logs_file = 'logs/logs_memory.json'
    
    # Checamos si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Agregamos los nuevos logs a la lista existente
    new_logs = [log.dict() for log in logs_proc]
    existing_logs.extend(new_logs)

    # Escribimos la lista de logs en el archivo logs.json
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"status": True, "msj": "Log de memoria guardado con exito"}