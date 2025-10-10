use worker::Application;
use worker::utils::APP_ADDRESS;

#[tokio::main]
async fn main() {
    let app = Application::build(APP_ADDRESS.as_ref())
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
