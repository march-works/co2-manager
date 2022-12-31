use server::infra::run_server;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    run_server().await.serve(addr).await?;

    Ok(())
}
