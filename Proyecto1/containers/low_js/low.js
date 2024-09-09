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
