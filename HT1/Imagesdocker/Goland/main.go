package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
)

// Define la estructura que se ajusta al JSON recibido
type StudentData struct {
    Student   string `json:"student"`
    Age      int    `json:"age"`
    Faculty  string `json:"faculty"`
    Disipline int   `json:"disipline"`
}

func handler(w http.ResponseWriter, r *http.Request) {
    if r.Method == http.MethodPost {
        var data StudentData

        // Decodifica el JSON del cuerpo de la solicitud
        err := json.NewDecoder(r.Body).Decode(&data)
        if err != nil {
            http.Error(w, "Error al decodificar JSON", http.StatusBadRequest)
            return
        }

        // Muestra los datos recibidos en la consola
        fmt.Printf("Recibido: %+v\n", data)

        // Envía una respuesta de éxito
        w.WriteHeader(http.StatusOK)
        fmt.Fprintf(w, "Datos recibidos correctamente")
    } else {
        http.Error(w, "Método no permitido", http.StatusMethodNotAllowed)
    }
}

func main() {
    http.HandleFunc("/data", handler)
    log.Println("Servidor escuchando en el puerto 8080...")
    log.Fatal(http.ListenAndServe(":8080", nil))
}
