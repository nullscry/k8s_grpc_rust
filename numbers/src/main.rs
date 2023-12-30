use tonic::{transport::Server, Request, Response, Status};

pub mod numbers_model {
    tonic::include_proto!("numbers");
}

use numbers_model::calculate_server::{Calculate, CalculateServer};
use numbers_model::{NumbersReply, NumbersRequest};

#[derive(Debug, Default)]
pub struct MyCalculate {}

#[tonic::async_trait]
impl Calculate for MyCalculate {
    async fn add_numbers(
        &self,
        request: Request<NumbersRequest>,
    ) -> Result<Response<NumbersReply>, Status> {
        println!("Got a request: {:?}", request);
        let numbers_request = request.into_inner();

        let reply = numbers_model::NumbersReply {
            result: numbers_request.lhs + numbers_request.rhs,
        };

        Ok(Response::new(reply))
    }

    async fn mult_numbers(
        &self,
        request: Request<NumbersRequest>,
    ) -> Result<Response<NumbersReply>, Status> {
        println!("Got a request: {:?}", request);
        let numbers_request = request.into_inner();

        let reply = numbers_model::NumbersReply {
            result: numbers_request.lhs * numbers_request.rhs,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "0.0.0.0:8888".parse()?;
    let mycalculate = MyCalculate::default();

    Server::builder()
        .add_service(CalculateServer::new(mycalculate))
        .serve(addr)
        .await?;

    Ok(())
}