#[path = "../../core.rs"]
mod core;
use crate::core::*;

use shared::*;

fn main() {
    setup_logging();

    use rena::prelude::client::GameNetClient as Client;
    let mut game_client: Client<RewarsClient> = Client::new();

    game_client.connect(([127, 0, 0, 1], 3306).into(), None, None);

    loop {
        if let Err(err) = game_client.tick() {
            use tracing::error;

            error!("{:?}", err);
        }
    }
}
