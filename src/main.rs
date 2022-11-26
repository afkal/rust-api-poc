use rust_api_poc::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
