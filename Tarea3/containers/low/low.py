# Script de bajo consumo usando Flask
from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World!'

if __name__ == '__main__':
    app.run(
        host='0.0.0.0',
        port=5000,           # Configura el puerto según tus necesidades
        threaded=True,      # Usa el modo de subprocesos para manejar múltiples solicitudes
        debug=False         # Desactiva el modo debug para producción
    )
