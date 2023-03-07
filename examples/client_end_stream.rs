mod api {
    tonic::include_proto!("service");
}

use std::{error::Error};

use api::service_client::ServiceClient;
use futures::{channel::mpsc::channel, SinkExt};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ServiceClient::connect("http://[::1]:10000").await?;
    for _ in 0..3 {
        do_operation(&mut client).await?;
    }
    Ok(())
}

async fn do_operation(client: &mut ServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let (mut snd, rcv) = channel(10);
    let mut res = client.send(rcv).await?.into_inner();
    snd.send(api::Request { expect_response: false }).await?;
    snd.send(api::Request { expect_response: true }).await?;
    res.message().await?;
    Ok(())
}
