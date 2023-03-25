pub mod gamemessage;
pub use gamemessage::*;

pub fn setup_logging() {
    use tracing::{info, Level};
    use tracing_subscriber::FmtSubscriber;

    let subscriber = if !cfg!(debug_assertions) {
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish()
    } else {
        FmtSubscriber::builder()
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(true)
            .with_max_level(Level::TRACE)
            .finish()
    };

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default failed");

    info!("started logging successfully");
    info!("debug build: {}", cfg!(debug_assertions));
}

pub const MAX_CLIENTS_PER_SERVER: usize = 16;
