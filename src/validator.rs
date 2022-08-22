use std::net::{Ipv4Addr, Ipv6Addr};

// Matches for characters
pub const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const DIGIT: &str = "0123456789";

pub const MAX_HOSTNAME_LENGTH: usize = 253;
pub const MAX_SHORTNAME_LENGTH: usize = 63;
pub const MAX_NICK_LENGTH: usize = 32; // RFC defines this as 9, but we ignore this since that is a
                                       // stupid limit

pub const NAME_FORBIDDEN: &str = "\0\r\n ";

fn matches_allowed(msg: &str, allowed: &str) -> bool {
    for character in msg.chars() {
        if !allowed.contains(character) {
            return false;
        }
    }
    true
}

fn matches_forbidden(msg: &str, forbidden: &str) -> bool {
    for character in msg.chars() {
        if forbidden.contains(character) {
            return false;
        }
    }
    true
}

pub fn valid_ipv4_addr(host_addr: &str) -> bool {
    host_addr.parse::<Ipv4Addr>().is_ok()
}

pub fn valid_ipv6_addr(host_addr: &str) -> bool {
    host_addr.parse::<Ipv6Addr>().is_ok()
}

pub fn valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > MAX_HOSTNAME_LENGTH {
        return false;
    }

    // hostname can be split up into shortnames with periods
    let toks: Vec<&str> = hostname.split('.').collect();
    for item in toks.iter() {
        if item.is_empty() || !valid_shortname(item) {
            return false;
        }
    }

    true
}

pub fn valid_shortname(shortname: &str) -> bool {
    if shortname.is_empty() || shortname.len() > MAX_SHORTNAME_LENGTH
    {
        return false;
    }

    let mut allowed = String::new();
    allowed.push_str(LOWER);
    allowed.push_str(DIGIT);
    allowed.push('-');
    matches_allowed(shortname, &allowed)
}

pub fn valid_nick(nick: &str) -> bool {
    if nick.is_empty() || nick.len() > MAX_NICK_LENGTH {
        return false;
    }
    let mut allowed = String::new();
    allowed.push_str(LETTERS);

    // Test first character (can not be digit or '-')
    let first: String = nick.chars().take(1).collect();
    if !matches_allowed(&first, &allowed) {
        return false;
    }
    // Test all characters (can be digits and '-')
    allowed.push_str(DIGIT);
    allowed.push('-');
    matches_allowed(nick, &allowed)
}
