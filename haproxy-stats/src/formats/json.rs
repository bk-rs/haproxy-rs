use serde::Deserialize;
use serde_json::Value as SerdeJsonValue;

#[derive(Deserialize, Debug, Clone)]
pub struct Field {
    pub pos: usize,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tags {
    pub origin: Box<str>,
    pub nature: Box<str>,
    pub scope: Box<str>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    #[serde(rename = "s32")]
    S32(i32),
    #[serde(rename = "s64")]
    S64(i64),
    #[serde(rename = "u32")]
    U32(u32),
    #[serde(rename = "u64")]
    U64(u64),
    #[serde(rename = "str")]
    Str(Box<str>),
}

impl Value {
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::S32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::S32(v) => Some(*v as i64),
            Self::S64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::U32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::U32(v) => Some(*v as u64),
            Self::U64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(v) => Some(v),
            _ => None,
        }
    }

    pub fn value_to_string(&self) -> String {
        match self {
            Self::S32(v) => v.to_string(),
            Self::S64(v) => v.to_string(),
            Self::U32(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
            Self::Str(v) => v.to_string(),
        }
    }
}

impl From<&Value> for SerdeJsonValue {
    fn from(v: &Value) -> Self {
        match v {
            Value::S32(v) => SerdeJsonValue::Number((*v).into()),
            Value::S64(v) => SerdeJsonValue::Number((*v).into()),
            Value::U32(v) => SerdeJsonValue::Number((*v).into()),
            Value::U64(v) => SerdeJsonValue::Number((*v).into()),
            Value::Str(v) => SerdeJsonValue::String(v.to_string()),
        }
    }
}
