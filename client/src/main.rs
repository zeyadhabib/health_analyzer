use clap::Parser;
use status::{ SpecsRequest, StatusRequest };
use status::status_service_client::StatusServiceClient;
use tonic::transport::{ Identity, ClientTlsConfig, Certificate, Channel };

mod status {
    tonic::include_proto!("status");
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Remote machine to monitor health stats.
    #[arg(short = 'a', long = "address", required = true)]
    address: String,

    /// Remote port.
    #[arg(short = 'p', long = "port", required = true)]
    port: u16,

    /// Remote machine domain name.
    #[arg(short = 'd', long = "domain", required = true)]
    domain: String,

    /// Number of health samples to collect per second.
    #[arg(short = 's', long = "sampling", required = true)]
    sampling: u64,

    /// Duration of monitoring in seconds.
    #[arg(short = 'D', long = "duration", required = true)]
    duration: u32
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let server_root_ca_cert = std::fs::read_to_string(r".\certs\chain.pem")?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.pem")?;
    let client_key = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.key")?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name(args.domain.as_str())
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let server_address = format!("{}:{}", args.address, args.port);
    println!("Connecting to {}", server_address);

    let channel = Channel::from_shared(server_address)
        .unwrap()
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = StatusServiceClient::new(channel);

    let request = tonic::Request::new(SpecsRequest {});
    let specs_response = client.get_specs(request).await.unwrap();
    println!("Response: {:?}", specs_response);

    let request = tonic::Request::new(StatusRequest {
        sampling_frequency: args.sampling,
        monitoring_duration: args.duration,
    });
    
    let mut stream = client.get_status(request).await.unwrap().into_inner();
    while let Some(status) = stream.message().await.unwrap() {
        println!("Response: {:?}", status);
    }

    Ok(())
}