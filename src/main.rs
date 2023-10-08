use new_heather::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = start_bot().await {
        error!("{err}");
    }
}
