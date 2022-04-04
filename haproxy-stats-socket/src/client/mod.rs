use core::fmt;
use std::{
    io::{Error as IoError, Read as _, Write as _},
    net::{SocketAddr, TcpStream},
    os::unix::net::UnixStream,
    path::Path,
};

use futures_util_either::Either;
use haproxy_stats::{Command, Commands};
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::{TcpStream as TokioTcpStream, UnixStream as TokioUnixStream},
};

//
mod impl_show_env;
mod impl_show_info;
mod impl_show_stat;

pub use impl_show_env::ClientShowEnvError;
pub use impl_show_info::ClientShowInfoError;
pub use impl_show_stat::ClientShowStatError;

//
pub struct Client {
    connect_info: ConnectInfo,
}

enum ConnectInfo {
    Tcp(SocketAddr),
    Unix(Box<Path>),
}

impl Client {
    pub fn with_tcp(addr: impl Into<SocketAddr>) -> Self {
        Self {
            connect_info: ConnectInfo::Tcp(addr.into()),
        }
    }

    pub fn with_unix(path: impl AsRef<Path>) -> Self {
        Self {
            connect_info: ConnectInfo::Unix(path.as_ref().into()),
        }
    }

    pub fn send(&self, command: &Command) -> Result<Vec<u8>, ClientSendError> {
        let write_bytes = command.to_write_bytes();

        //
        let mut stream = match &self.connect_info {
            ConnectInfo::Tcp(addr) => {
                Either::Left(TcpStream::connect(addr).map_err(ClientSendError::ConnectFailed)?)
            }
            ConnectInfo::Unix(path) => {
                Either::Right(UnixStream::connect(path).map_err(ClientSendError::ConnectFailed)?)
            }
        };

        //
        stream
            .write_all(&write_bytes[..])
            .map_err(ClientSendError::WriteFailed)?;

        //
        let mut response: Vec<u8> = Vec::with_capacity(2048);
        stream
            .read_to_end(&mut response)
            .map_err(ClientSendError::ReadFailed)?;

        Ok(response)
    }

    pub async fn send_async(&self, command: &Command) -> Result<Vec<u8>, ClientSendError> {
        let write_bytes = command.to_write_bytes();

        //
        let mut stream = match &self.connect_info {
            ConnectInfo::Tcp(addr) => Either::Left(
                TokioTcpStream::connect(addr)
                    .await
                    .map_err(ClientSendError::ConnectFailed)?,
            ),
            ConnectInfo::Unix(path) => Either::Right(
                TokioUnixStream::connect(path)
                    .await
                    .map_err(ClientSendError::ConnectFailed)?,
            ),
        };

        //
        stream
            .write_all(&write_bytes[..])
            .await
            .map_err(ClientSendError::WriteFailed)?;

        //
        let mut response: Vec<u8> = Vec::with_capacity(2048);
        let mut buf = vec![0; 2048];
        loop {
            let n = stream
                .read(&mut buf)
                .await
                .map_err(ClientSendError::ReadFailed)?;

            if n == 0 {
                break;
            }
            response.extend_from_slice(&buf[..n]);
        }

        Ok(response)
    }

    pub fn send_multiple(&self, commands: Commands<'_>) -> Result<Vec<Vec<u8>>, ClientSendError> {
        let _write_bytes = commands.to_write_bytes();

        todo!()
    }

    pub async fn send_multiple_async(
        &self,
        commands: Commands<'_>,
    ) -> Result<Vec<Vec<u8>>, ClientSendError> {
        let _write_bytes = commands.to_write_bytes();

        todo!()
    }
}

//
//
//
#[derive(Debug)]
pub enum ClientSendError {
    ConnectFailed(IoError),
    WriteFailed(IoError),
    ReadFailed(IoError),
}

impl fmt::Display for ClientSendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ClientSendError {}
