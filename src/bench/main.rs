use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use temperature::temperature_service_client::TemperatureServiceClient;

pub mod temperature {
    tonic::include_proto!("temperature");
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("grpc_command");

    for number_of_calls in [100, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(number_of_calls),
            &number_of_calls,
            |b, &n_calls| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let mut client = create_grpc_client().await;
                        for _ in 0..n_calls {
                            client
                                .set_temperature(temperature::RequestedTemperature { value: 10.0 })
                                .await
                                .unwrap();
                        }
                    })
            },
        );
    }

    group.finish();
}

async fn create_grpc_client() -> TemperatureServiceClient<tonic::transport::Channel> {
    let channel = tonic::transport::Channel::from_static("http://localhost:5000")
        .connect()
        .await
        .expect("Can't create a channel");

    TemperatureServiceClient::new(channel)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
