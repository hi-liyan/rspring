use rspring_core::*;

#[rspring_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}