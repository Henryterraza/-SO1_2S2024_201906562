package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/segmentio/kafka-go"
)

const (
	kafkaBroker = "my-cluster-kafka-bootstrap:9092" // Dirección del broker de Kafka
	groupID     = "winner_consumers"                // Grupo de consumidores
)

// Función que inicia el consumidor Kafka y procesa mensajes desde múltiples tópicos
func consumeKafkaMessages(topic string) {
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers:        []string{kafkaBroker},
		Topic:          topic,
		GroupID:        groupID,
		MinBytes:       10e3,
		MaxBytes:       10e6,
		CommitInterval: time.Second,
	})
	defer reader.Close()

	fmt.Printf("Consumer is listening for messages on topic: %s\n", topic)

	for {
		msg, err := reader.ReadMessage(context.Background())
		if err != nil {
			log.Printf("Error reading message from topic %s: %v", topic, err)
			fmt.Printf("Error reading message from topic %s: %v\n", topic, err)
			break
		}
		log.Printf("Mensaje recibido de %s - Key: %s, Value: %s", topic, string(msg.Key), string(msg.Value))
		fmt.Printf("Mensaje recibido de %s - Key: %s, Value: %s\n", topic, string(msg.Key), string(msg.Value))
	}
}

func main() {
	// Iniciar un consumidor para el tópico "winners"
	go consumeKafkaMessages("winners")

	// Mantener la aplicación activa
	select {}
}
