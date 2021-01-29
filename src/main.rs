use log::debug;

mod arguments;
mod error;
mod storage;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let arguments::Arguments {
        port,
        user,
        password_hash,
    } = arguments::parse();

    debug!(
        "user: {}, password_hash: {}, port: {}",
        user, password_hash, port
    );

    storage::init().unwrap();
}
