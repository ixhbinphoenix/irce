use std::{error, fmt};
use crate::validator::{valid_hostname, valid_ipv4_addr, valid_ipv6_addr, valid_nick};

#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    NoCommand,
    InvalidSource(String),
    InvalidNick(String),
    InvalidUser(String),
    InvalidHost(String),
    Unimplemented
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyMessage => write!(f, "Empty message"),
            ParseError::NoCommand => write!(f, "Message contains no command"),
            ParseError::InvalidSource(source) => write!(f, "Invalid source: {}", &source),
            ParseError::InvalidNick(nick) => write!(f, "Invalid nick: {}", &nick),
            ParseError::InvalidUser(user) => write!(f, "Invalid user: {}", &user),
            ParseError::InvalidHost(host) => write!(f, "Invalid host: {}", &host),
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
    Nick(String),
    NickHost(String, HostType),
    NickUserHost(String, String, HostType),
    Host(HostType),
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
        Some(parse_source(vec[0])?)
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
            // 1st element ([0]) is the command so we can push a "16th" [15] argument
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

// "Source" is one of the following formats:
// - <hostname> -> MsgSource::Host
// - <nick>[!<user>][@<host>] -> MsgSource::Nick OR MsgSource::NickHost OR MsgSource::NickUserHost
fn parse_source(source: &str)-> Result<MsgSource, ParseError> {
    let vec: Vec<&str> = source.splitn(2, '@').collect();

    if !vec[1].is_empty() {
        let host = parse_host(vec[1])?;

        if vec[0].contains('!') {
            // This is an ugly solution, but a working one.
            let vec_two: Vec<&str> = vec[0].splitn(2, '!').collect();

            if valid_nick(vec_two[0]) {
                // TODO: Validate user. I have no idea how to do that, there are no contraints
                // defined anywhere
                return Ok(MsgSource::NickUserHost(vec_two[0].into(), vec_two[1].into(), host));
            } else {
                return Err(ParseError::InvalidNick(vec_two[0].into()));
            }
        } else {
            if valid_nick(vec[0]) {
                return Ok(MsgSource::NickHost(vec[0].into(), host));
            } else {
                return Err(ParseError::InvalidNick(vec[0].into()));
            }
        }
    } else {
        if let Ok(host) = parse_host(source) {
            return Ok(MsgSource::Host(host));
        } else if valid_nick(source) {
            return Ok(MsgSource::Nick(source.into()));
        } else {
            return Err(ParseError::InvalidSource(source.into()));
        }
    }
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
