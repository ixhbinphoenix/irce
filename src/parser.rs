use std::{error, fmt};
use crate::validator::{valid_hostname, valid_ipv4_addr, valid_ipv6_addr};

#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    NoCommand,
    InvalidHost(String),
    Unimplemented
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyMessage => write!(f, "Empty message"),
            ParseError::NoCommand => write!(f, "Message contains no command"),
            ParseError::InvalidHost(host) => write!(f, "Invalid gost string, {}", &host),
            ParseError::Unimplemented => write!(f, "Unimplemented")
        }
    }
}

impl error::Error for ParseError {} 

#[derive(Debug)]
pub enum HostType {
    HostName(String),
    IPv4(String),
    IPv6(String)
}

#[derive(Debug)]
pub enum MsgSource {
    Host(HostType)
}

#[derive(Debug)]
pub struct ParsedMsg {
    pub source: Option<MsgSource>,
    pub command: String,
    pub params: Vec<String>
}

pub fn parse_message(message: &str) -> Result<ParsedMsg, ParseError> {
    let mut line = message;
    if line.is_empty() {
        return Err(ParseError::EmptyMessage)
    }

    let source = if line.starts_with(":") {
        let vec: Vec<&str> = line.splitn(2, ' ').collect();
        if vec.len() < 2 {
            return Err(ParseError::NoCommand);
        }
        line = vec[1];
        // TODO: Parse message prefix
        let hostname = vec[0];
        match parse_host(hostname) {
            Ok(host) => {
                Some(MsgSource::Host(host))
            },
            Err(e) => {
                return Err(e);
            },
        }
    } else {
        None
    };

    let mut params: Vec<String> = Vec::new();
    let mut n_args: u8 = 0;
    // Example: USER phoenix 0 * :phoenix
    // into: ["USER", "phoenix", "0", "*", "phoenix"]
    loop {
        let vec: Vec<&str> = line.splitn(2, ' ').collect();
        n_args += 1;
        params.push(vec[0].to_string());
        if vec.len() < 2 {
            break;
        }

        line = vec[1];
        if line.is_empty() {
            break;
        } else if &line[..1] == ":" {
            line = &line[1..line.len()];
            params.push(line.to_string());
            break;
        } else if n_args > 15 {
            // As per old RFC definitions, the max amount of arguments is 15
            params.push(line.to_string());
            break;
        }
    }

    let command = params.remove(0);

    Ok(ParsedMsg {
        source,
        command,
        params
    })
}

fn parse_host(hostname: &str) -> Result<HostType, ParseError> {
    if valid_ipv4_addr(hostname) {
        Ok(HostType::IPv4(hostname.into()))
    } else if valid_ipv6_addr(hostname) {
        Ok(HostType::IPv6(hostname.into()))
    } else if valid_hostname(hostname) {
        Ok(HostType::HostName(hostname.into()))
    } else {
        Err(ParseError::InvalidHost(hostname.into()))
    }
}
