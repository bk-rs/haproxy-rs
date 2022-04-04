use core::fmt;

use haproxy_stats::{stat::StatisticsFromCsvBytesError, Command, Statistic, Statistics};

use super::{Client, ClientSendError};

//
impl Client {
    pub async fn show_stat(&self) -> Result<Vec<Statistic>, ClientShowStatError> {
        let response = self
            .send_async(&Command::show_stat())
            .await
            .map_err(ClientShowStatError::ClientSendError)?;

        let statistics = Statistics::from_csv_bytes(response)
            .map_err(ClientShowStatError::ResponseParseError)?;

        Ok(statistics.0)
    }
}

//
#[derive(Debug)]
pub enum ClientShowStatError {
    ClientSendError(ClientSendError),
    ResponseParseError(StatisticsFromCsvBytesError),
}

impl fmt::Display for ClientShowStatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ClientShowStatError {}
