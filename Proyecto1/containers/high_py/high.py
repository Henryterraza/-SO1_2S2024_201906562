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
