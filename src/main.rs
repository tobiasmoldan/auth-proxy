use log::debug;
use shadow_rs::new;

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

    storage::get_api("test").unwrap();
    storage::new_api("test", &storage::Api::default())
        .await
        .unwrap();
    println!("{:?}", storage::get_api("test").unwrap().unwrap());
}
