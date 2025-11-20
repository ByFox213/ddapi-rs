use serde_derive::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Addr {
    pub ip: IpAddr,
    pub port: u16,
    pub protocol: Protocol,
}

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
        match s {
            "tw-0.5+udp" => Protocol::V5,
            "tw-0.6+udp" => Protocol::V6,
            "tw-0.7+udp" => Protocol::V7,
            "ddrs-0.1+quic" => Protocol::VPg,
            _ => panic!("Unknown protocol: {}", s),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for Addr {
    type Error = String;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = url.split("://").collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid URL format, expected 'protocol://host:port', got: {}",
                url
            ));
        }

        let protocol_str = parts[0];
        let host_port = parts[1];

        let protocol = Protocol::from(protocol_str);

        let (host, port_str) = if host_port.starts_with('[') {
            let parts: Vec<&str> = host_port.splitn(2, ']').collect();
            if parts.len() != 2 {
                return Err(format!(
                    "Invalid IPv6 format, expected '[host]:port', got: {}",
                    host_port
                ));
            }
            let host = &parts[0][1..];
            let port_part = parts[1];
            if !port_part.starts_with(':') {
                return Err(format!(
                    "Invalid IPv6 port format, expected ']:port', got: {}",
                    host_port
                ));
            }
            let port_str = &port_part[1..];
            (host, port_str)
        } else {
            let parts: Vec<&str> = host_port.split(':').collect();
            if parts.len() != 2 {
                return Err(format!(
                    "Invalid host:port format, expected 'host:port', got: {}",
                    host_port
                ));
            }
            (parts[0], parts[1])
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
        let mut addrs = Vec::new();

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
