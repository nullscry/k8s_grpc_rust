use tonic::{transport::Server, Request, Response, Status};

pub mod clock_model {
    tonic::include_proto!("clock");
}
use chrono::prelude::*;
use clock_model::clock_server::{Clock, ClockServer};
use clock_model::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyClock {}

#[tonic::async_trait]
impl Clock for MyClock {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = clock_model::HelloReply {
            message: format!("Hello {} from Clock! Time is {}", request.into_inner().name, Utc::now().to_string()).into(),
        };

        Ok(Response::new(reply))
    }

    async fn say_hello_again(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = clock_model::HelloReply {
            message: format!("Hello AGAIN {} from Clock! Time is {}", request.into_inner().name, Utc::now().to_string()).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8888".parse()?;
    let clock = MyClock::default();

    Server::builder()
        .add_service(ClockServer::new(clock))
        .serve(addr)
        .await?;

    Ok(())
}