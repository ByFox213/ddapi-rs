use serde_derive::{Deserialize, Serialize};
use std::net::IpAddr;

/// Teeworlds/DDNet server address, including protocol.
///
/// # Examples
/// ```rust
/// use ddapi_rs::prelude::Addr;
///
/// let a = Addr::try_from("tw-0.7+udp://127.0.0.1:8303").unwrap();
/// assert_eq!(a.port, 8303);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Addr {
    pub ip: IpAddr,
    pub port: u16,
    pub protocol: Protocol,
}

/// Supported protocol identifiers used by DDNet master responses.
///
/// # Examples
/// ```rust
/// use ddapi_rs::prelude::Protocol;
///
/// assert_eq!(Protocol::V7.as_str(), "tw-0.7+udp");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Protocol {
    V5,
    V6,
    V7,
    VPg,
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::V5 => "tw-0.5+udp",
            Protocol::V6 => "tw-0.6+udp",
            Protocol::V7 => "tw-0.7+udp",
            Protocol::VPg => "ddrs-0.1+quic",
        }
    }
}

impl From<&str> for Protocol {
    fn from(s: &str) -> Self {
        Protocol::try_from_str(s).unwrap_or_else(|e| panic!("{e}"))
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Protocol {
    pub fn try_from_str(value: &str) -> Result<Self, String> {
        match value {
            "tw-0.5+udp" => Ok(Protocol::V5),
            "tw-0.6+udp" => Ok(Protocol::V6),
            "tw-0.7+udp" => Ok(Protocol::V7),
            "ddrs-0.1+quic" => Ok(Protocol::VPg),
            _ => Err(format!("Unknown protocol: {value}")),
        }
    }
}

impl TryFrom<&str> for Addr {
    type Error = String;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let (protocol_str, host_port) = url.split_once("://").ok_or_else(|| {
            format!(
                "Invalid URL format, expected 'protocol://host:port', got: {}",
                url
            )
        })?;

        let protocol = Protocol::try_from_str(protocol_str)?;

        let (host, port_str) = if let Some(rest) = host_port.strip_prefix('[') {
            let (host, rest) = rest.split_once(']').ok_or_else(|| {
                format!(
                    "Invalid IPv6 format, expected '[host]:port', got: {}",
                    host_port
                )
            })?;
            let port_str = rest.strip_prefix(':').ok_or_else(|| {
                format!(
                    "Invalid IPv6 port format, expected ']:port', got: {}",
                    host_port
                )
            })?;
            (host, port_str)
        } else {
            host_port.split_once(':').ok_or_else(|| {
                format!(
                    "Invalid host:port format, expected 'host:port', got: {}",
                    host_port
                )
            })?
        };

        let ip: IpAddr = host
            .parse()
            .map_err(|e| format!("Failed to parse IP '{}': {}", host, e))?;

        let port: u16 = port_str
            .parse()
            .map_err(|e| format!("Failed to parse port '{}': {}", port_str, e))?;

        Ok(Addr { ip, port, protocol })
    }
}

pub mod addr_serialization {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(addrs: &[Addr], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(addrs.len()))?;
        for addr in addrs {
            let url = format!("{}://{}:{}", addr.protocol.as_str(), addr.ip, addr.port);
            seq.serialize_element(&url)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Addr>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let string_vec: Vec<String> = Vec::deserialize(deserializer)?;
        let mut addrs = Vec::with_capacity(string_vec.len());

        for s in string_vec {
            match Addr::try_from(s.as_str()) {
                Ok(addr) => addrs.push(addr),
                Err(e) => {
                    return Err(D::Error::custom(format!(
                        "Failed to parse address '{}': {}",
                        s, e
                    )))
                }
            }
        }

        Ok(addrs)
    }
}
