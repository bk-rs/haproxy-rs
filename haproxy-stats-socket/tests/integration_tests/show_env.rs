use std::error;

use haproxy_stats_socket::client::Client;

use super::helpers::{get_tcp_addr, init_logger};

#[tokio::test]
async fn show_env() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    //
    let client = Client::with_tcp(get_tcp_addr()?);

    let env_vars = client.show_env().await?;
    println!("env_vars {:?}", env_vars);

    Ok(())
}
