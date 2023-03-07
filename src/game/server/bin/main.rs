#[path ="../mod.rs"]
mod server;
use server::*;
#[path ="../../shared/mod.rs"]
mod shared;
use shared::*;

fn main() {
    setup_logging();

    use redengine::prelude::server::GameNetServer as Server;
    let game_server: Server<RewarsServer> = Server::new();
}
