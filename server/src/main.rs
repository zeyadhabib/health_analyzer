
pub mod device_status;
pub mod status {
    tonic::include_proto!("status");
}

use tokio::sync::mpsc;
use sysinfo::{ System, SystemExt };
use std::{ pin::Pin, time::Duration };
use tokio_stream::{ wrappers::ReceiverStream, Stream };
use status::status_service_server::{ StatusServiceServer ,StatusService };
use status::{ SpecsRequest, SpecsResponse, StatusRequest, StatusResponse };
use tonic::{ transport::{ Server, Identity, ServerTlsConfig }, Request, Response, Status };


use device_status::{ get_status, get_specs }; 

#[derive(Debug, Default)]
pub struct GetStatusService {}

#[tonic::async_trait]
impl StatusService for GetStatusService {

    type GetStatusStream = Pin<Box<dyn Stream<Item = Result<StatusResponse, Status>> + Send>>;

    async fn get_specs (&self , _request: Request<SpecsRequest>) -> Result<Response<SpecsResponse>, Status> {
        println!("Got a request: {:?}", _request);
        let reply = get_specs();
        Ok(Response::new(reply))
    }

    async fn get_status (&self , _request: Request<StatusRequest>) -> Result<Response<Self::GetStatusStream>, Status> {
        println!("Got a request: {:?}", _request);
        let (tx, rx) = mpsc::channel(32);

        let mut sys = System::new_all();
        sys.refresh_all();

        // Calculate the number of iterations per second, time to sleep between iterations, and the total number of iterations
        let iterations_per_sec = _request.get_ref().sampling_frequency;
        let sleep_duration = Duration::from_secs_f64(1.0 / iterations_per_sec as f64);
        let iterations = (_request.get_ref().monitoring_duration as u64) * iterations_per_sec;

        println!("Iterations per second: {}", iterations_per_sec);
        println!("Sleep duration: {:?}", sleep_duration);
        println!("Iterations: {}", iterations);

        // Spawn a new task to send the status every sleep_duration
        tokio::spawn(async move {
            for _ in 0..iterations {
                let reply = get_status();
                tx.send(Ok(reply)).await.unwrap();
                tokio::time::sleep(sleep_duration).await;
            }
        });

        // Return the stream
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    } 
}
    

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let key = std::fs::read_to_string(r".\certs\server-leaf\server-leaf.key")?;
    let cert = std::fs::read_to_string(r".\certs\server-leaf\server-leaf.pem")?;
    let identity = Identity::from_pem(cert, key);
    let status_service = GetStatusService::default();

    let tls = ServerTlsConfig::new()
        .identity(identity);

    Server::builder()
        .tls_config(tls)?
        .add_service(StatusServiceServer::new(status_service))
        .serve(addr)
        .await?;

    Ok(())
}
