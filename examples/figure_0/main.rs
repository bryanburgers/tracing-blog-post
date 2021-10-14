use tracing::info;
use tracing_subscriber::prelude::*;

mod custom_layer;
use custom_layer::CustomLayer;

fn main() {
    // Set up how `tracing-subscriber` will deal with tracing data.
    tracing_subscriber::registry().with(CustomLayer).init();

    // Log something simple. In `tracing` parlance, this creates an "event".
    info!(a_bool = true, answer = 42, message = "first example");
}
