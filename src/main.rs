use log::debug;

mod arguments;
mod error;
mod storage;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let arguments::Arguments {
        addr,
        user,
        password_hash,
    } = arguments::parse();

    debug!(
        "user: {}, password_hash: {}, addr: {}",
        user, password_hash, addr
    );

    storage::init().unwrap();
}
