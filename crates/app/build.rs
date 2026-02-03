use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../frontend/src");
    println!("cargo:rerun-if-changed=../frontend/package.json");
    println!("cargo:rerun-if-changed=../frontend/index.html");
    println!("cargo:rerun-if-changed=../frontend/vite.config.ts");

    // Only build frontend in release mode
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    if profile != "release" {
        println!("cargo:warning=Skipping frontend build in debug mode. Use --release for integrated build.");
        return;
    }

    // Get the frontend directory path
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let frontend_dir = manifest_dir
        .parent()
        .unwrap() // crates
        .parent()
        .unwrap() // project root
        .join("frontend");

    if !frontend_dir.exists() {
        println!(
            "cargo:warning=Frontend directory not found at {:?}",
            frontend_dir
        );
        return;
    }

    println!("cargo:warning=Building frontend...");

    // Check if npm is available
    let npm_check = Command::new("npm").arg("--version").output();

    match npm_check {
        Ok(_) => {}
        Err(e) => {
            println!(
                "cargo:warning=npm not found: {}. Skipping frontend build.",
                e
            );
            println!("cargo:warning=To build frontend, ensure Node.js and npm are installed.");
            return;
        }
    }

    // Install dependencies if needed
    let node_modules = frontend_dir.join("node_modules");
    if !node_modules.exists() {
        println!("cargo:warning=Installing frontend dependencies...");
        let install_status = Command::new("npm")
            .arg("install")
            .current_dir(&frontend_dir)
            .status();

        match install_status {
            Ok(status) if status.success() => {}
            Ok(status) => {
                println!("cargo:warning=npm install failed with status: {}", status);
                return;
            }
            Err(e) => {
                println!("cargo:warning=Failed to run npm install: {}", e);
                return;
            }
        }
    }

    // Build frontend
    let build_status = Command::new("npm")
        .args(["run", "build"])
        .current_dir(&frontend_dir)
        .status();

    match build_status {
        Ok(status) if status.success() => {
            println!("cargo:warning=Frontend built successfully");

            // Set rerun trigger for dist directory
            let dist_dir = manifest_dir
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("dist");
            if dist_dir.exists() {
                println!("cargo:rerun-if-changed={}", dist_dir.display());
            }
        }
        Ok(status) => {
            println!(
                "cargo:warning=Frontend build failed with status: {}",
                status
            );
            println!(
                "cargo:warning=You can still run the server, but frontend will not be available."
            );
        }
        Err(e) => {
            println!("cargo:warning=Failed to run frontend build: {}", e);
            println!(
                "cargo:warning=You can still run the server, but frontend will not be available."
            );
        }
    }
}
