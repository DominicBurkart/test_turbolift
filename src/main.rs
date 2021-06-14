/// Tests that [turbolift](https://dominic.computer/turbolift) when built from cargo
/// instead of from source. Like all libs using proc_macro2's unstable features,
/// at writing this project needs to be run with semver exemption, like
/// `RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run`.
use std::time::{SystemTime, Instant};
use std::error::Error;

// Instantiate cluster interface.
lazy_static! {
    static ref K8S: Mutex<K8s> = Mutex::new(
        K8s::with_deploy_function_and_max_replicas(
            Box::new(load_container_into_kind),
            2
        )
    );
}

/// Tells the cluster to make images accessible by sending them
/// to kind (https://kind.sigs.k8s.io/).
fn load_container_into_kind(tag: &str) -> anyhow::Result<&str> {
    std::process::Command::new("kind")
        .args(
            format!("load docker-image {}", tag)
                .as_str()
                .split(' ')
        )
        .status()?;
    Ok(tag)
}

/// function to distribute. Note this non-async call is turned into
/// an async call.
#[on(K8S)]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!(hello().await);
    Ok(())
}
