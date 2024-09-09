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
