use core::fmt;

use haproxy_stats::{info::InfoFromKvBytesError, Command, Info};

use super::{Client, ClientSendError};

//
impl Client {
    pub async fn show_info(&self) -> Result<Info, ClientShowInfoError> {
        let response = self
            .send_async(&Command::show_info())
            .await
            .map_err(ClientShowInfoError::ClientSendError)?;

        let info =
            Info::from_kv_bytes(response).map_err(ClientShowInfoError::ResponseParseError)?;

        Ok(info)
    }
}

//
#[derive(Debug)]
pub enum ClientShowInfoError {
    ClientSendError(ClientSendError),
    ResponseParseError(InfoFromKvBytesError),
}

impl fmt::Display for ClientShowInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ClientShowInfoError {}
