use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = app::run_server().await {
        eprintln!("Server error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
