use core::{fmt, ops::Deref};

use csv::{Error as CsvError, ReaderBuilder};
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use serde_json::{Error as SerdeJsonError, Map, Value};

use crate::formats::json;

//
pub const SVNAME_FRONTEND: &str = "FRONTEND";
pub const SVNAME_BACKEND: &str = "BACKEND";

//
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Statistic {
    #[serde(rename = "0")]
    Frontend(FrontendStatistic),
    #[serde(rename = "1")]
    Backend(BackendStatistic),
    #[serde(rename = "2")]
    Server(ServerStatistic),
    #[serde(rename = "3")]
    Listener(ListenerStatistic),
}

impl Statistic {
    pub fn as_frontend(&self) -> Option<&FrontendStatistic> {
        match self {
            Self::Frontend(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_backend(&self) -> Option<&BackendStatistic> {
        match self {
            Self::Backend(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_server(&self) -> Option<&ServerStatistic> {
        match self {
            Self::Server(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_listener(&self) -> Option<&ListenerStatistic> {
        match self {
            Self::Listener(s) => Some(s),
            _ => None,
        }
    }
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct ListenerStatistic {
    pub pxname: Box<str>,
    pub svname: Box<str>,
    //
    pub scur: usize,
    pub smax: usize,
    pub slim: usize,
    pub stot: usize,
    //
    pub bin: usize,
    pub bout: usize,
    //
    pub dreq: usize,
    pub dresp: usize,
    //
    pub ereq: usize,
    //
    pub status: Status,
    //
    pub pid: usize,
    pub iid: usize,
    pub sid: usize,
    // TODO,
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct FrontendStatistic {
    pub pxname: Box<str>,
    // svname SKIP
    //
    pub scur: usize,
    pub smax: usize,
    pub slim: usize,
    pub stot: usize,
    //
    pub bin: usize,
    pub bout: usize,
    //
    pub dreq: usize,
    pub dresp: usize,
    //
    pub ereq: usize,
    //
    pub status: Status,
    //
    pub pid: usize,
    pub iid: usize,
    //
    pub rate: usize,
    pub rate_lim: usize,
    pub rate_max: usize,
    //
    pub hrsp_1xx: Option<usize>,
    pub hrsp_2xx: Option<usize>,
    pub hrsp_3xx: Option<usize>,
    pub hrsp_4xx: Option<usize>,
    pub hrsp_5xx: Option<usize>,
    pub hrsp_other: Option<usize>,
    //
    pub req_rate: Option<usize>,
    pub req_rate_max: Option<usize>,
    pub req_tot: Option<usize>,
    // TODO,
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct BackendStatistic {
    pub pxname: Box<str>,
    // svname SKIP
    //
    pub qcur: usize,
    pub qmax: usize,
    //
    pub scur: usize,
    pub smax: usize,
    pub slim: usize,
    pub stot: usize,
    //
    pub bin: usize,
    pub bout: usize,
    //
    pub dreq: usize,
    pub dresp: usize,
    //
    pub econ: usize,
    pub eresp: usize,
    //
    pub wretr: usize,
    pub wredis: usize,
    //
    pub status: Status,
    //
    pub weight: usize,
    //
    pub act: usize,
    pub bck: usize,
    //
    pub chkdown: usize,
    pub lastchg: usize,
    // Because missing maybe in 1.7.9
    pub downtime: Option<usize>,
    //
    pub pid: usize,
    pub iid: usize,
    //
    pub lbtot: usize,
    //
    pub rate: usize,
    pub rate_max: usize,
    //
    pub hrsp_1xx: Option<usize>,
    pub hrsp_2xx: Option<usize>,
    pub hrsp_3xx: Option<usize>,
    pub hrsp_4xx: Option<usize>,
    pub hrsp_5xx: Option<usize>,
    pub hrsp_other: Option<usize>,
    //
    pub req_tot: Option<usize>,
    //
    pub cli_abrt: usize,
    pub srv_abrt: usize,
    // TODO,
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct ServerStatistic {
    pub pxname: Box<str>,
    pub svname: Box<str>,
    //
    pub qcur: usize,
    pub qmax: usize,
    //
    pub scur: usize,
    pub smax: usize,
    pub slim: Option<usize>,
    pub stot: usize,
    //
    pub bin: usize,
    pub bout: usize,
    //
    pub dresp: usize,
    //
    pub econ: usize,
    pub eresp: usize,
    //
    pub wretr: usize,
    pub wredis: usize,
    //
    pub status: Status,
    //
    pub weight: usize,
    //
    pub act: usize,
    pub bck: usize,
    //
    pub chkfail: Option<usize>,
    pub chkdown: Option<usize>,
    pub lastchg: Option<usize>,
    pub downtime: Option<usize>,
    //
    pub qlimit: Option<usize>,
    //
    pub pid: usize,
    pub iid: usize,
    pub sid: usize,
    //
    pub throttle: Option<usize>,
    //
    pub lbtot: usize,
    //
    pub tracked: Option<usize>,
    //
    pub rate: usize,
    pub rate_max: usize,
    //
    pub check_status: Option<CheckStatus>,
    //
    pub check_code: Option<usize>,
    pub check_duration: Option<usize>,
    //
    pub hrsp_1xx: Option<usize>,
    pub hrsp_2xx: Option<usize>,
    pub hrsp_3xx: Option<usize>,
    pub hrsp_4xx: Option<usize>,
    pub hrsp_5xx: Option<usize>,
    pub hrsp_other: Option<usize>,
    //
    // Because missing in json format
    #[serde(default)]
    pub hanafail: Box<str>,
    //
    pub cli_abrt: usize,
    pub srv_abrt: usize,
    //
    // TODO,
}

//
//
//
#[derive(Deserialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[serde(rename = "UP")]
    UP,
    #[serde(rename = "DOWN")]
    DOWN,
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(other)]
    Other(String),
}

#[derive(Deserialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    #[serde(rename = "L4TOUT")]
    L4TOUT,
    #[serde(rename = "* L4TOUT")]
    LastL4TOUT,
    #[serde(rename = "L4CON")]
    L4CON,
    #[serde(rename = "* L4CON")]
    LastL4CON,
    #[serde(rename = "L7OK")]
    L7OK,
    #[serde(rename = "* L7OK")]
    LastL7OK,
    #[serde(other)]
    Other(String),
}

//
//
//
#[derive(Debug, Clone)]
pub struct Statistics(pub Vec<Statistic>);

impl Deref for Statistics {
    type Target = Vec<Statistic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Statistics {
    pub fn from_csv_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, StatisticsFromCsvBytesError> {
        let bytes = bytes.as_ref();
        if &bytes[..1] != b"#" {
            return Err(StatisticsFromCsvBytesError::Other(
                "The first line begins with a sharp ('#')",
            ));
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&bytes[1..]);
        let mut iter = rdr.records();

        //
        let (header, header_names) = if let Some(record) = iter.next() {
            let mut record = record.map_err(StatisticsFromCsvBytesError::CsvParseFailed)?;
            record.trim();
            let list: Vec<Box<str>> = record
                .deserialize(None)
                .map_err(StatisticsFromCsvBytesError::HeaderDeFailed)?;
            (record, list)
        } else {
            return Err(StatisticsFromCsvBytesError::HeaderMissing);
        };

        let header_type_position = header_names
            .iter()
            .position(|x| *x == "type".into())
            .ok_or(StatisticsFromCsvBytesError::HeaderNameMismatch(
                "type missing",
            ))?;
        let header_svname_position = header_names
            .iter()
            .position(|x| *x == "svname".into())
            .ok_or(StatisticsFromCsvBytesError::HeaderNameMismatch(
                "svname missing",
            ))?;

        let mut inner = vec![];
        for (i, record) in iter.enumerate() {
            let record = record.map_err(StatisticsFromCsvBytesError::CsvParseFailed)?;

            let r#type = record.get(header_type_position).ok_or_else(|| {
                StatisticsFromCsvBytesError::RowValueMismatch(
                    format!(
                        "line:{} position:{} type missing",
                        i + 1,
                        header_type_position
                    )
                    .into(),
                )
            })?;
            let svname = record.get(header_svname_position).ok_or_else(|| {
                StatisticsFromCsvBytesError::RowValueMismatch(
                    format!(
                        "line:{} position:{} svname missing",
                        i + 1,
                        header_svname_position
                    )
                    .into(),
                )
            })?;

            match r#type {
                "0" => {
                    if svname != SVNAME_FRONTEND {
                        return Err(StatisticsFromCsvBytesError::RowValueMismatch(
                            format!(
                                "line:{} svname:{} svname should eq {}",
                                i + 1,
                                svname,
                                SVNAME_FRONTEND,
                            )
                            .into(),
                        ));
                    }

                    let row: FrontendStatistic = record
                        .deserialize(Some(&header))
                        .map_err(StatisticsFromCsvBytesError::RowDeFailed)?;
                    inner.push(Statistic::Frontend(row));
                }
                "1" => {
                    if svname != SVNAME_BACKEND {
                        return Err(StatisticsFromCsvBytesError::RowValueMismatch(
                            format!(
                                "line:{} svname:{} svname should eq {}",
                                i + 1,
                                svname,
                                SVNAME_BACKEND,
                            )
                            .into(),
                        ));
                    }

                    let row: BackendStatistic = record
                        .deserialize(Some(&header))
                        .map_err(StatisticsFromCsvBytesError::RowDeFailed)?;
                    inner.push(Statistic::Backend(row));
                }
                "2" => {
                    let row: ServerStatistic = record
                        .deserialize(Some(&header))
                        .map_err(StatisticsFromCsvBytesError::RowDeFailed)?;
                    inner.push(Statistic::Server(row));
                }
                "4" => {
                    let row: ListenerStatistic = record
                        .deserialize(Some(&header))
                        .map_err(StatisticsFromCsvBytesError::RowDeFailed)?;
                    inner.push(Statistic::Listener(row));
                }
                _ => return Err(StatisticsFromCsvBytesError::UnknownType),
            }
        }

        Ok(Self(inner))
    }

    pub fn from_json_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, StatisticsFromJsonBytesError> {
        let bytes = bytes.as_ref();

        let output = serde_json::from_slice::<JsonOutput>(bytes)
            .map_err(StatisticsFromJsonBytesError::DeOutputFailed)?;

        let array: Vec<Value> = output
            .0
            .into_iter()
            .map(|x| {
                Value::Object(
                    x.into_iter()
                        .map(|y| {
                            let v = match y.field.name.as_ref() {
                                "type" => Value::from(y.value.value_to_string()),
                                _ => Value::from(&y.value),
                            };

                            (y.field.name.to_string(), v)
                        })
                        .collect::<Map<String, Value>>(),
                )
            })
            .collect();

        let inner = serde_json::from_value::<Vec<Statistic>>(Value::Array(array))
            .map_err(StatisticsFromJsonBytesError::DeFailed)?;

        Ok(Self(inner))
    }
}

//
#[derive(Debug)]
pub enum StatisticsFromCsvBytesError {
    CsvParseFailed(CsvError),
    HeaderMissing,
    HeaderDeFailed(CsvError),
    HeaderNameMismatch(&'static str),
    RowDeFailed(CsvError),
    RowValueMismatch(Box<str>),
    UnknownType,
    Other(&'static str),
}

impl fmt::Display for StatisticsFromCsvBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StatisticsFromCsvBytesError {}

//
#[derive(Debug)]
pub enum StatisticsFromJsonBytesError {
    DeOutputFailed(SerdeJsonError),
    DeFailed(SerdeJsonError),
}

impl fmt::Display for StatisticsFromJsonBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StatisticsFromJsonBytesError {}

//
//
//
#[derive(Deserialize, Debug, Clone)]
pub struct JsonOutput(pub Vec<Vec<JsonOutputItem>>);

#[derive(Deserialize, Debug, Clone)]
pub struct JsonOutputItem {
    #[serde(rename = "objType")]
    pub obj_type: Box<str>,
    #[serde(rename = "proxyId")]
    pub proxy_id: usize,
    pub id: usize,
    pub field: json::Field,
    #[serde(rename = "processNum")]
    pub process_num: usize,
    pub tags: json::Tags,
    pub value: json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_statistics_from_csv_bytes() {
        let bytes = include_bytes!("../tests/files/2_5_5_show_stat.csv");

        let statistics = Statistics::from_csv_bytes(bytes).unwrap();

        assert_eq!(statistics.len(), 12);
        assert_eq!(
            statistics[0].as_frontend().unwrap().pxname,
            "http-frontend".into()
        );
        assert_eq!(
            statistics[1].as_server().unwrap().svname,
            "http-backend-srv-1".into()
        );
        assert_eq!(
            statistics[2].as_backend().unwrap().pxname,
            "http-backend".into()
        );
    }

    #[test]
    fn test_statistics_from_csv_bytes_with_match_files() {
        for entry in fs::read_dir("tests/files").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.to_str().unwrap().ends_with("show_stat.csv") {
                let bytes = fs::read(&path).unwrap();

                let _ = Statistics::from_csv_bytes(bytes).unwrap();

                println!("file {:?}", path);
            }
        }
    }

    #[test]
    fn test_statistics_from_json_bytes() {
        let bytes = include_bytes!("../tests/files/2_5_5_show_stat.json");

        let statistics = Statistics::from_json_bytes(bytes).unwrap();

        assert_eq!(statistics.len(), 12);
        assert_eq!(
            statistics[0].as_frontend().unwrap().pxname,
            "http-frontend".into()
        );
        assert_eq!(
            statistics[1].as_server().unwrap().svname,
            "http-backend-srv-1".into()
        );
        assert_eq!(
            statistics[2].as_backend().unwrap().pxname,
            "http-backend".into()
        );
    }
}
