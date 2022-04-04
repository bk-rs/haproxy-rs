use std::error;

use haproxy_stats_socket::client::Client;

use super::helpers::{get_tcp_addr, init_logger};

#[tokio::test]
async fn show_info() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    //
    let client = Client::with_tcp(get_tcp_addr()?);

    let info = client.show_info().await?;
    println!("info {:?}", info);

    Ok(())
}
