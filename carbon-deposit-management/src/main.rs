use server::application::{run_server, subscribe_queues};

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse()?;
    subscribe_queues();
    run_server().await.serve(addr).await?;

    Ok(())
}
