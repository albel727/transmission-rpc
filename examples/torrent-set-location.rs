extern crate transmission_rpc;

use dotenv::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse};
use transmission_rpc::types::{Id, Nothing};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let basic_auth = BasicAuth {
        user: env::var("TUSER")?,
        password: env::var("TPWD")?,
    };
    let client = TransClient::with_auth(&url, basic_auth);
    let res: RpcResponse<Nothing> = client.torrent_set_location(
            vec![Id::Id(1)],
            String::from("/new/location"),
            Option::from(false),
        ).await?;
    println!("Set-location result: {:?}", &res.is_ok());

    Ok(())
}
