mod api {
    tonic::include_proto!("service");
}

use std::error::Error;

use api::service_server::{Service as ServiceTrait, ServiceServer};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Debug)]
struct Service;

#[tonic::async_trait]
impl ServiceTrait for Service {
    type SendStream = Receiver<Result<api::Response, Status>>;

    async fn send(
        &self,
        request: Request<Streaming<api::Request>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        let (mut snd, rcv) = channel(10);
        let mut req_stream = request.into_inner();
        tokio::spawn(async move {
            while let Some(message) = req_stream.next().await {
                match message {
                    Ok(req) => {
                        if req.expect_response {
                            snd.send(Ok(api::Response {}))
                                .await
                                .expect("should succeed send");
                        }
                    }
                    Err(err) => {
                        log::error!("caught error: {err:?}");
                    }
                }
            }
        });
        Ok(Response::new(rcv))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let addr = "[::1]:10000".parse().unwrap();
    let svc = ServiceServer::new(Service);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
