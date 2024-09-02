# Script de alto consumo de CPU
import time
import math

while True:
    for _ in range(5000000):  # Aumenta la cantidad de iteraciones
        list1 = [math.sqrt(i) for i in range(10000)]  # Maneja una lista más grande
        list2 = [math.sin(i) for i in list1]  # Añade otro cálculo que consuma CPU
        result = sum(list2)  # Suma los resultados para hacer más cálculos
    time.sleep(0.05)