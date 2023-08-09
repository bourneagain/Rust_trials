use std::io::stdin;

use edgeaction::{
    edge_action_client::EdgeActionClient, EdgeActionRequest};

pub mod edgeaction {
    tonic::include_proto!("edgeaction");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EdgeActionClient::connect("http://[::1]:8080").await?;
        println!("\nPlease input");
        let mut u = String::new();
        stdin().read_line(&mut u).unwrap();
        let u = u.trim();
        let request = tonic::Request::new(EdgeActionRequest {
            input: String::from(u),
        });
        let response = client.edge_action_rpc(request).await?;
        println!("Got: '{}' from service", response.into_inner().result);
    Ok(())
}
