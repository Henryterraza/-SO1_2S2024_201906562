package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"net"
	"time"

	pb "natacion/packagestu" // Importa el paquete generado (ajusta con la ruta correcta)

	"github.com/segmentio/kafka-go"

	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedDisciplineServiceServer
}

func (s *server) SendStudent(ctx context.Context, req *pb.StudentInfo) (*pb.DisciplineResponse, error) {
	log.Printf("Received student: %s, Age: %d, Faculty: %s, Discipline: %d",
		req.Student, req.Age, req.Faculty, req.Discipline)

	// Determina si el estudiante es ganador o perdedor
	result := determineWinner()
	var topic string
	if result {
		topic = "winners"
	} else {
		topic = "losers"
	}

	// Enviar a Kafka
	if err := sendToKafka(topic, req); err != nil {
		log.Printf("Error sending to Kafka topic %s: %v", topic, err)
		return nil, err
	}

	// Respuesta a enviar al cliente
	message := fmt.Sprintf("Estudiante %s de la facultad %s recibido en la disciplina %d, es %s",
		req.Student, req.Faculty, req.Discipline, topic)
	return &pb.DisciplineResponse{Message: message}, nil
}

// determineWinner simula un lanzamiento de moneda para decidir si el estudiante es ganador (true) o perdedor (false)
func determineWinner() bool {
	rand.Seed(time.Now().UnixNano())
	return rand.Intn(2) == 0
}

// sendToKafka envía la información del estudiante al tópico de Kafka
func sendToKafka(topic string, student *pb.StudentInfo) error {
	writer := kafka.NewWriter(kafka.WriterConfig{
		Brokers: []string{"my-cluster-kafka-bootstrap:9092"}, // Configura la dirección del broker de Kafka
		Topic:   topic,
	})
	defer writer.Close()

	// Crear el mensaje a enviar
	message := kafka.Message{
		Key: []byte(student.Student),
		Value: []byte(fmt.Sprintf("Student: %s, Age: %d, Faculty: %s, Discipline: %d",
			student.Student, student.Age, student.Faculty, student.Discipline)),
	}

	// Enviar el mensaje
	return writer.WriteMessages(context.Background(), message)
}

func main() {
	// Escuchar en el puerto 50051
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen on port 50051: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterDisciplineServiceServer(grpcServer, &server{})

	log.Println("diciplina gRPC server is running on port 50051...")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Failed to serve gRPC server: %v", err)
	}
}
