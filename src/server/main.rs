use std::time::Duration;
use temperature::temperature_service_server::{TemperatureService, TemperatureServiceServer};
use tonic::transport::Server;

pub mod temperature {
    tonic::include_proto!("temperature");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5000".parse()?;

    let svc = TemperatureServiceServer::new(TemperatureServiceImpl {});
    tokio::time::timeout(Duration::from_secs(60), Server::builder().add_service(svc).serve(addr)).await;

    Ok(())
}

struct TemperatureServiceImpl;

#[tonic::async_trait]
impl TemperatureService for TemperatureServiceImpl {
    fn set_temperature<'life0, 'async_trait>(
        &'life0 self,
        _request: tonic::Request<temperature::RequestedTemperature>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<tonic::Response<()>, tonic::Status>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(futures::future::ok(tonic::Response::new(())))
    }
}
