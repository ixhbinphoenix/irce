use std::net::{Ipv4Addr, Ipv6Addr};

// Matches for characters
pub const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const DIGIT: &str = "0123456789";

pub const MAX_HOSTNAME_LENGTH: usize = 253;
pub const MAX_SHORTNAME_LENGTH: usize = 63;

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
    match host_addr.parse::<Ipv4Addr>() {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn valid_ipv6_addr(host_addr: &str) -> bool {
    match host_addr.parse::<Ipv6Addr>() {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > MAX_HOSTNAME_LENGTH {
        return false;
    }

    // hostname can be tokenised with periods, ever string enclosed should be a valid shortname
    let toks: Vec<&str> = hostname.split('.').collect();
    for item in toks.iter() {
        if item.is_empty() || !valid_shortname(item) {
            return false;
        }
    }

    true
}

pub fn valid_shortname(shortname: &str) -> bool {
    if shortname.is_empty()
       || shortname.len() > MAX_SHORTNAME_LENGTH
    {
        return false;
    }

    let mut allowed = String::new();
    allowed.push_str(LOWER);
    allowed.push_str(DIGIT);
    allowed.push_str("-");
    matches_allowed(shortname, &allowed)
}
