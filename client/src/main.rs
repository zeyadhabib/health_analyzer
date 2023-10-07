
use status::{ SpecsRequest, StatusRequest };
use status::status_service_client::StatusServiceClient;
use tonic::transport::{ Identity, ClientTlsConfig, Certificate, Channel };

mod status {
    tonic::include_proto!("status");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_root_ca_cert = std::fs::read_to_string(r".\certs\chain.pem")?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.pem")?;
    let client_key = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.key")?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("zeyad.server.com")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::from_static("https://[::1]:50051")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = StatusServiceClient::new(channel);

    let request = tonic::Request::new(SpecsRequest {});
    let specs_response = client.get_specs(request).await.unwrap();
    println!("Response: {:?}", specs_response);

    let request = tonic::Request::new(StatusRequest {
        sampling_frequency: 100,
        monitoring_duration: 5,
    });
    
    let mut stream = client.get_status(request).await.unwrap().into_inner();
    while let Some(status) = stream.message().await.unwrap() {
        println!("Response: {:?}", status);
    }

    Ok(())
}