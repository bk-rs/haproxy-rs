use std::{error, io::ErrorKind as IoErrorKind};

use haproxy_stats_socket::{
    client::{Client, ClientSendError, ClientShowStatError},
    haproxy_stats::{Command, Statistics},
};

use super::helpers::{get_tcp_addr, get_unix_path, init_logger};

#[tokio::test]
async fn show_stat() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    //
    let client = Client::with_unix(get_unix_path()?);
    match client.show_stat().await {
        Ok(statistics) => {
            println!("statistics {:?}", statistics)
        }
        Err(ClientShowStatError::ClientSendError(ClientSendError::ConnectFailed(err)))
            if err.kind() == IoErrorKind::PermissionDenied =>
        {
            // nothing
        }
        Err(err) => {
            return Err(err.into());
        }
    }

    //
    let client = Client::with_tcp(get_tcp_addr()?);

    let statistics = client.show_stat().await?;
    println!("statistics {:?}", statistics);

    let res = client.send(&Command::show_stat())?;
    let _ = Statistics::from_csv_bytes(res)?;

    let res = client.send_async(&Command::show_stat()).await?;
    let _ = Statistics::from_csv_bytes(res)?;

    Ok(())
}
