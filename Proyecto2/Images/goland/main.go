package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	pb "goland/packagestu" // Actualiza con el path de tu paquete gRPC

	"google.golang.org/grpc"
)

type Student struct {
	Name       string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func sendStudentData(ch chan Student, targetService string) {
	for student := range ch {
		// Crear conexión gRPC con el servicio de disciplina
		// fmt.Printf("Conectando a %s...\n", targetService)
		conn, err := grpc.Dial(targetService, grpc.WithInsecure(), grpc.WithBlock(), grpc.WithTimeout(2*time.Second))
		if err != nil {
			log.Printf("No se pudo conectar al servicio %s: %v", targetService, err)
			continue // Continuar con el siguiente estudiante
		}

		client := pb.NewDisciplineServiceClient(conn)

		request := &pb.StudentInfo{
			Student:    student.Name,
			Age:        int32(student.Age),
			Faculty:    student.Faculty,
			Discipline: int32(student.Discipline),
		}

		// Enviar solicitud gRPC con timeout
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		response, err := client.SendStudent(ctx, request)
		if err != nil {
			log.Printf("Error al enviar datos del estudiante a %s: %v", targetService, err)
			continue // Continuar con el siguiente estudiante
		}
		fmt.Printf("Respuesta de disciplina %d: %s\n", student.Discipline, response.Message)
	}
}

func studentHandler(w http.ResponseWriter, r *http.Request, natacionCh chan Student, atletismoCh chan Student, boxeoCh chan Student) {
	var student Student

	if err := json.NewDecoder(r.Body).Decode(&student); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	fmt.Printf("Received student: %+v\n", student)

	// Enviar el estudiante al canal correspondiente según la disciplina
	switch student.Discipline {
	case 1:
		natacionCh <- student
	case 2:
		atletismoCh <- student
	case 3:
		boxeoCh <- student
	default:
		http.Error(w, "Disciplina no válida", http.StatusBadRequest)
		return
	}

	// Responder con el mismo JSON
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(student)
}

func main() {
	natacionCh := make(chan Student)
	atletismoCh := make(chan Student)
	boxeoCh := make(chan Student)

	// Iniciar goroutines para cada disciplina
	go sendStudentData(natacionCh, "natacion-svc:50051")
	go sendStudentData(atletismoCh, "atletismo-svc:50051")
	go sendStudentData(boxeoCh, "boxeo-svc:50051")

	http.HandleFunc("/golang-service", func(w http.ResponseWriter, r *http.Request) {
		studentHandler(w, r, natacionCh, atletismoCh, boxeoCh)
	})

	log.Println("Server starting on port 3030...")
	if err := http.ListenAndServe(":3030", nil); err != nil {
		log.Fatal(err)
	}

	defer close(natacionCh)
	defer close(atletismoCh)
	defer close(boxeoCh)
}
