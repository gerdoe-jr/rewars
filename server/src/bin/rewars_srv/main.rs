#[path = "../../core.rs"]
mod core;
use crate::core::*;

use shared::*;

fn main() {
    setup_logging();

    use rena::prelude::server::GameNetServer as Server;
    let mut game_server: Server<RewarsServer> = Server::new();

    game_server.listen(3306);

    loop {
        if let Err(err) = game_server.tick() {
            use tracing::error;

            error!("{:?}", err);
        }
    }
}
