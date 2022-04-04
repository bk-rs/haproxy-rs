use core::fmt;

use haproxy_stats::{env::EnvironmentVariablesFromKvBytesError, Command, EnvironmentVariables};

use super::{Client, ClientSendError};

//
impl Client {
    pub async fn show_env(&self) -> Result<EnvironmentVariables, ClientShowEnvError> {
        let response = self
            .send_async(&Command::show_env())
            .await
            .map_err(ClientShowEnvError::ClientSendError)?;

        let info = EnvironmentVariables::from_kv_bytes(response)
            .map_err(ClientShowEnvError::ResponseParseError)?;

        Ok(info)
    }
}

//
#[derive(Debug)]
pub enum ClientShowEnvError {
    ClientSendError(ClientSendError),
    ResponseParseError(EnvironmentVariablesFromKvBytesError),
}

impl fmt::Display for ClientShowEnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ClientShowEnvError {}
