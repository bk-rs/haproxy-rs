use core::{fmt, time::Duration};
use std::io::{BufRead as _, Cursor, Error as IoError};

use chrono::NaiveDate;
use csv::{ByteRecord, Error as CsvError};
use duration_str::deserialize_duration;
use semver::Version;
use serde::Deserialize;
use serde_json::{Error as SerdeJsonError, Map, Value};

use crate::formats::json;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Info {
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Version")]
    pub version: Version,
    #[serde(rename = "Release_date")]
    pub release_date: NaiveDate,
    #[serde(rename = "Nbproc")]
    pub nbproc: usize,
    #[serde(rename = "Process_num")]
    pub process_num: usize,
    #[serde(rename = "Pid")]
    pub pid: usize,
    #[serde(rename = "Uptime")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub uptime: Duration,
    #[serde(rename = "Uptime_sec")]
    pub uptime_sec: usize,
    // TODO,
}

impl Info {
    pub fn from_json_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, InfoFromJsonBytesError> {
        let bytes = bytes.as_ref();

        let output = serde_json::from_slice::<JsonOutput>(bytes)
            .map_err(InfoFromJsonBytesError::DeOutputFailed)?;

        let map: Map<String, Value> = output
            .0
            .into_iter()
            .map(|x| {
                let v = match x.field.name.as_ref() {
                    "Release_date" => {
                        Value::from(x.value.as_str().unwrap_or_default().replacen('/', "-", 3))
                    }
                    _ => Value::from(&x.value),
                };

                (x.field.name.to_string(), v)
            })
            .collect();

        serde_json::from_value::<Self>(Value::Object(map)).map_err(InfoFromJsonBytesError::DeFailed)
    }

    pub fn from_kv_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, InfoFromKvBytesError> {
        let bytes = bytes.as_ref();

        let cursor = Cursor::new(bytes);

        let list: Vec<(Box<str>, Box<str>)> = cursor
            .lines()
            .map(|x| match x {
                Ok(s) => {
                    let mut split = s.split(": ");
                    let k: Box<str> = split.next().unwrap_or_default().into();
                    let v: Box<str> = split.next().unwrap_or_default().into();

                    let v = match k.as_ref() {
                        "Release_date" => v.replacen('/', "-", 3).into(),
                        _ => v,
                    };

                    Ok((k, v))
                }
                Err(err) => Err(InfoFromKvBytesError::LinesReadFailed(err)),
            })
            .collect::<Result<_, _>>()?;

        let header = list.iter().map(|(x, _)| x.as_ref()).collect::<Vec<_>>();
        let header_record = ByteRecord::from(header);

        let row = list.iter().map(|(_, x)| x.as_ref()).collect::<Vec<_>>();
        let row_record = ByteRecord::from(row);

        row_record
            .deserialize::<Self>(Some(&header_record))
            .map_err(InfoFromKvBytesError::ValueDeFailed)
    }
}

//
#[derive(Debug)]
pub enum InfoFromJsonBytesError {
    DeOutputFailed(SerdeJsonError),
    DeFailed(SerdeJsonError),
}

impl fmt::Display for InfoFromJsonBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InfoFromJsonBytesError {}

//
#[derive(Debug)]
pub enum InfoFromKvBytesError {
    LinesReadFailed(IoError),
    ValueDeFailed(CsvError),
}

impl fmt::Display for InfoFromKvBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InfoFromKvBytesError {}

//
//
//
#[derive(Deserialize, Debug, Clone)]
pub struct JsonOutput(pub Vec<JsonOutputItem>);

#[derive(Deserialize, Debug, Clone)]
pub struct JsonOutputItem {
    pub field: json::Field,
    #[serde(rename = "processNum")]
    pub process_num: usize,
    pub tags: json::Tags,
    pub value: json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_from_json_bytes() {
        let bytes = include_bytes!("../tests/files/2_5_5_show_info.json");

        let info = Info::from_json_bytes(bytes).unwrap();

        println!("{:?}", info);

        assert_eq!(info.name, "HAProxy".into());
    }

    #[test]
    fn test_info_from_kv_bytes() {
        let bytes = include_bytes!("../tests/files/2_5_5_show_info.txt");

        let info = Info::from_kv_bytes(bytes).unwrap();

        assert_eq!(info.name, "HAProxy".into());
    }
}
