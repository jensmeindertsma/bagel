use std::{fs::File, process::Termination};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(File::create("/tmp/bagel.log").expect("writer should be initialized"))
        .init();
}
