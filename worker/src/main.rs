use worker::{Application, utils::prod::APP_ADDRESS};

#[tokio::main]
async fn main() {
    let app = Application::build(APP_ADDRESS)
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
