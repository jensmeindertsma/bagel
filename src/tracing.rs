use std::fs::File;

use tracing::Level;

pub fn setup_tracing() {
    let file = File::create("/tmp/bagel.log").unwrap();

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(file)
        .init();
}
