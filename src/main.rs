/// Tests that [turbolift](https://dominic.computer/turbolift) when built from cargo
/// instead of from source. Like all libs using proc_macro2's unstable features,
/// at writing this project needs to be run with semver exemption, like
/// `RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run`.

use std::error::Error;

#[macro_use]
extern crate lazy_static;
use tokio::sync::Mutex;
use turbolift::{on, kubernetes::K8s, tracing};

// Instantiate cluster interface.
lazy_static! {
    static ref K8S: Mutex<K8s> = Mutex::new(
        K8s::new(
            Box::new(load_container_into_kind),
            2
        )
    );
}

/// Tells the cluster to make images accessible by sending them
/// to kind (https://kind.sigs.k8s.io/).
fn load_container_into_kind(tag: String) -> anyhow::Result<String> {
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
/// an async call that returns an anyhow::Result<String> instead of
/// a String.
#[on(K8S)]
fn hello() -> String {
    "World".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello {}!", hello().await?);
    Ok(())
}
