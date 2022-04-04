use std::{
    env, error,
    net::{IpAddr, SocketAddr},
};

use log::debug;

pub(super) fn get_tcp_addr() -> Result<SocketAddr, Box<dyn error::Error>> {
    let port = env::var("HAPROXY_STATS_SOCKET_TCP_PORT")?;
    debug!("HAPROXY_STATS_SOCKET_TCP_PORT {}", port);

    let ip_addr = "127.0.0.1".parse::<IpAddr>()?;
    let port = port.parse::<u16>()?;

    Ok(SocketAddr::new(ip_addr, port))
}

pub(super) fn get_unix_path() -> Result<String, Box<dyn error::Error>> {
    let path = env::var("HAPROXY_STATS_SOCKET_UNIX_PATH")?;
    debug!("HAPROXY_STATS_SOCKET_UNIX_PATH {}", path);

    Ok(path)
}

pub(super) fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
