use serde::{
    de::{self, value, DeserializeOwned},
    ser::SerializeSeq,
    Deserialize, Serialize,
};
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum RespValue {
    SimpleStrings(String),
    Error(String),
    Integer(i64),
    BulkStrings(Option<Vec<u8>>),
    Array(Vec<RespValue>),
}

impl Serialize for RespValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RespValue::SimpleStrings(s) => serializer.serialize_str(&format!("+{}\r\n", s)),
            RespValue::Error(e) => serializer.serialize_str(&format!("-{}\r\n", e)),
            RespValue::Integer(i) => serializer.serialize_str(&format!(":{}\r\n", i)),
            RespValue::BulkStrings(Some(data)) => {
                let s = format!("${}\r\n", data.len());
                let mut result = s.into_bytes();
                result.extend(data);
                result.extend(b"\r\n");
                serializer.serialize_bytes(&result)
            }
            RespValue::BulkStrings(None) => serializer.serialize_str("$-1\r\n"),
            RespValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for value in arr {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for RespValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RespValueVisitor;

        impl<'de> de::Visitor<'de> for RespValueVisitor {
            type Value = RespValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid RESP value")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.get(0) {
                    Some(b'+') => self.parse_simple_string(v).map_err(E::custom),
                    Some(b'-') => self.parse_error(v).map_err(E::custom),
                    Some(b':') => self.parse_integer(v).map_err(E::custom),
                    Some(b'$') => self.parse_bulk_string(v).map_err(E::custom),
                    Some(b'*') => self.parse_array(v).map_err(E::custom),
                    _ => Err(E::custom("invalid RESP value")),
                }
            }
        }

        impl RespValueVisitor {
            fn parse_simple_string(&self, v: &[u8]) -> Result<RespValue, String> {
                let (content, _) = Self::split_at_crlf(&v[1..])
                    .ok_or_else(|| "invalid simple string format".to_string())?;
                let s = str::from_utf8(content)
                    .map_err(|_| "invalid UTF-8 in simple string".to_string())?;
                Ok(RespValue::SimpleStrings(s.to_string()))
            }

            fn parse_error(&self, v: &[u8]) -> Result<RespValue, String> {
                let (content, _) = Self::split_at_crlf(&v[1..])
                    .ok_or_else(|| "invalid error format".to_string())?;
                let s =
                    str::from_utf8(content).map_err(|_| "invalid UTF-8 in error".to_string())?;
                Ok(RespValue::Error(s.to_string()))
            }

            fn parse_integer(&self, v: &[u8]) -> Result<RespValue, String> {
                let (content, _) = Self::split_at_crlf(&v[1..])
                    .ok_or_else(|| "invalid integer format".to_string())?;
                let s =
                    str::from_utf8(content).map_err(|_| "invalid UTF-8 in integer".to_string())?;
                let i = s
                    .parse::<i64>()
                    .map_err(|_| "invalid integer".to_string())?;
                Ok(RespValue::Integer(i))
            }

            fn parse_bulk_string(&self, v: &[u8]) -> Result<RespValue, String> {
                let (len_str, rest) = Self::split_at_crlf(&v[1..])
                    .ok_or_else(|| "invalid bulk string format".to_string())?;
                let len = str::from_utf8(len_str)
                    .map_err(|_| "invalid bulk string length".to_string())?
                    .parse::<i32>()
                    .map_err(|_| "invalid bulk string length".to_string())?;

                if len == -1 {
                    Ok(RespValue::BulkStrings(None))
                } else if len >= 0 {
                    let data_len = len as usize;
                    if rest.len() < data_len + 2 {
                        return Err("incomplete bulk string data".to_string());
                    }
                    if rest[data_len] != b'\r' || rest[data_len + 1] != b'\n' {
                        return Err("bulk string not terminated with CRLF".to_string());
                    }
                    let data = rest[..data_len].to_vec();
                    Ok(RespValue::BulkStrings(Some(data)))
                } else {
                    Err("invalid bulk string length".to_string())
                }
            }

            fn parse_array(&self, v: &[u8]) -> Result<RespValue, String> {
                let (len_str, mut rest) = Self::split_at_crlf(&v[1..])
                    .ok_or_else(|| "invalid array format".to_string())?;
                let len = str::from_utf8(len_str)
                    .map_err(|_| "invalid array length".to_string())?
                    .parse::<usize>()
                    .map_err(|_| "invalid array length".to_string())?;

                let mut array = Vec::with_capacity(len);
                for _ in 0..len {
                    let (element, remaining) = self.parse_resp_sequence(rest)?;
                    array.push(element);
                    rest = remaining;
                }

                if (!rest.is_empty()) {
                    return Err("invalid RESP value".to_string());
                }

                Ok(RespValue::Array(array))
            }

            fn parse_resp_sequence<'a>(
                &self,
                v: &'a [u8],
            ) -> Result<(RespValue, &'a [u8]), String> {
                let resp_value = match v.get(0) {
                    Some(b'+') => self.parse_simple_string(v)?,
                    Some(b'-') => self.parse_error(v)?,
                    Some(b':') => self.parse_integer(v)?,
                    Some(b'$') => self.parse_bulk_string(v)?,
                    Some(b'*') => self.parse_array(v)?,
                    _ => {
                        return Err("invalid RESP value".to_string());
                    }
                };
                let len = self.resp_value_len(&resp_value);
                Ok((resp_value, &v[len..]))
            }

            fn resp_value_len(&self, value: &RespValue) -> usize {
                match value {
                    RespValue::SimpleStrings(s) | RespValue::Error(s) => s.len() + 3, // +/-data\r\n
                    RespValue::Integer(i) => i.to_string().len() + 3,                 // :num\r\n
                    RespValue::BulkStrings(Some(data)) => {
                        data.len() + 5 + data.len().to_string().len()
                    } // $len\r\ndata\r\n
                    RespValue::BulkStrings(None) => 5,                                // $-1\r\n
                    RespValue::Array(arr) => {
                        let mut len = 1 + arr.len().to_string().len() + 2; // *len\r\n
                        for elem in arr {
                            len += self.resp_value_len(elem);
                        }
                        len
                    }
                }
            }

            fn split_at_crlf(data: &[u8]) -> Option<(&[u8], &[u8])> {
                let mut i = 0;
                while i < data.len() - 1 {
                    if data[i] == b'\r' && data[i + 1] == b'\n' {
                        return Some((&data[..i], &data[i + 2..]));
                    }
                    i += 1;
                }
                None
            }
        }

        deserializer.deserialize_str(RespValueVisitor)
    }
}
