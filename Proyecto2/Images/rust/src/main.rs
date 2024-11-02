use serde::Deserialize;
use tokio::task;
use tonic::transport::Channel;
use discipline::discipline_service_client::DisciplineServiceClient;
use discipline::{StudentInfo, DisciplineResponse};
use warp::Filter;
use std::net::IpAddr; // Importar IpAddr
use std::net::SocketAddr; // Importar SocketAddr


pub mod discipline {
    tonic::include_proto!("discipline");
}

#[derive(Deserialize, Debug)]
struct Student {
    student: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

#[tokio::main]
async fn main() {

    let local_ip: IpAddr = "0.0.0.0".parse().unwrap();
    let port = 8080;
    let addr: SocketAddr = SocketAddr::new(local_ip, port);
    
    println!("Servidor corriendo en: {}", addr);

    warp::serve(warp::post()
        .and(warp::path("rust-service"))
        .and(warp::body::json())
        .map(|student: Student| {
            println!("Received student: {:?}", student);

            // Spawn a thread to handle the gRPC call
            let _handle = task::spawn(async move {
                let response = match student.discipline {
                    1 => send_to_discipline("http://natacion-svc:50051", student).await,
                    2 => send_to_discipline("http://atletismo-svc:50051", student).await,
                    3 => send_to_discipline("http://boxeo-svc:50051", student).await,
                    _ => Err("Invalid discipline".into())
                };
                
                if let Ok(msg) = response {
                    println!("Response from discipline: {}", msg);
                } else {
                    eprintln!("Failed to send to discipline: {:?}", response);
                }
            });

            warp::reply::with_status("Student received", warp::http::StatusCode::OK)
        }))
        .run(addr)
        .await;
}

async fn send_to_discipline(url: &str, student: Student) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("Connecting to discipline service at: {}", url);
    let mut client = DisciplineServiceClient::connect(url.to_string()).await?;
    
    let request = tonic::Request::new(StudentInfo {
        student: student.student,
        age: student.age,
        faculty: student.faculty,
        discipline: student.discipline,
    });

    let response = client.send_student(request).await?;
    Ok(response.into_inner().message)
}
