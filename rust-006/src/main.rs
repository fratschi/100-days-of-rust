use tracing::{info, debug};
use tracing_subscriber;

fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");
    debug!("this is a debug {}", "message");

    info!("{{ \"msg\": \"{}\" }}", "preparing to shave yaks");
}