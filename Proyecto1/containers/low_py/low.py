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
