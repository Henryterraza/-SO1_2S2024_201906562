from locust import HttpUser, task

class MyUser(HttpUser):
    @task
    def send_request(self):
        # Definir el JSON que se enviará
        student_data = {
            "student": "Henry Terraza",
            "age": 20,
            "faculty": "Ingenieria",
            "disipline": 1
        }

        # Enviar la solicitud POST con el JSON a la ruta /data
        self.client.post("/data", json=student_data)