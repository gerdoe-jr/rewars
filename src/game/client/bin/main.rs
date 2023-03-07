#[path ="../mod.rs"]
mod client;
use client::*;
#[path ="../../shared/mod.rs"]
mod shared;
use shared::*;

fn main() {
    setup_logging();

    use redengine::prelude::client::GameNetClient as Client;
    let game_client: Client<RewarsClient> = Client::new();
}
