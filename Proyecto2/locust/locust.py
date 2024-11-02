from locust import HttpUser, task, between
import random

class StudentLoadTest(HttpUser):
    # Espera entre cada tarea para simular un comportamiento humano
    wait_time = between(1, 5)

    @task
    def send_student_data(self):
        # Generar datos de prueba para cada estudiante
        student_data = {
            "student": random.choice(["Juan Pedro", "Maria Lopez", "Carlos Sanchez", "Ana Jimenez"]),
            "age": random.randint(18, 25),
            "faculty": random.choice(["Ingenieria", "Agronomia"]),
            "discipline": random.choice([1, 2, 3])  # 1=Natación, 2=Atletismo, 3=Boxeo
        }
        
        # # Enviar solicitud POST con el JSON generado
        # headers = {"Content-Type": "application/json"}
        # self.client.post("/golang-service", json=student_data, headers=headers)

        if student_data["faculty"] == "Ingenieria":
            url = "/rust-service"
        else:
            url = "/golang-service"
        
        # Enviar solicitud POST con el JSON generado
        headers = {"Content-Type": "application/json"}
        self.client.post(url, json=student_data, headers=headers)
