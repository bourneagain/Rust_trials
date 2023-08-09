use tonic::{transport::Server, Request, Response, Status};
use edgeaction::{
    EdgeActionRequest,
    EdgeActionResponse,
    edge_action_server::{EdgeAction, EdgeActionServer}};

pub mod edgeaction {
  tonic::include_proto!("edgeaction");
}

#[derive(Debug, Default)]
pub struct EdgeActionSvc {}


#[tonic::async_trait]
impl EdgeAction for EdgeActionSvc {
  async fn edge_action_rpc(&self, request: Request<EdgeActionRequest>) -> Result<Response<EdgeActionResponse>, Status> {
    println!("Received request from: {:?}", request);

   let response = edgeaction::EdgeActionResponse {
       result: format!("Hello {}!", request.into_inner().input).into(),
   };

        Ok(Response::new(response))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "[::1]:8080".parse().unwrap();
  let edgeactionsvc = EdgeActionSvc::default();

  Server::builder().add_service(EdgeActionServer::new(edgeactionsvc))
    .serve(address)
    .await?;
  Ok(())

}
