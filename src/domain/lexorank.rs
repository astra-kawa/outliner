use crate::domain::DomainError;

const RANK_LENGTH: usize = 12;

pub struct LexoRank {
    rank: String,
    num: u64,
}

impl LexoRank {
    pub fn new(rank: &str) -> Result<Self, DomainError> {
        if rank.len() != RANK_LENGTH {
            return Err(DomainError::InvalidRankLength);
        }

        let num = from_str(rank).map_err(|_| DomainError::InvalidRank)?;

        Ok(LexoRank {
            rank: rank.to_string(),
            num: num,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseBase36Error {
    Empty,
    InvalidChar(char),
    Overflow,
}

#[inline]
fn val_of(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'a'..='z' => Some(10 + (c as u8 - b'a')),
        'A'..='Z' => Some(10 + (c as u8 - b'A')),
        _ => None,
    }
}

pub fn from_str(s: &str) -> Result<u64, ParseBase36Error> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseBase36Error::Empty);
    }

    let mut acc: u64 = 0;
    for ch in s.chars() {
        let v = val_of(ch).ok_or(ParseBase36Error::InvalidChar(ch))? as u64;
        acc = acc.checked_mul(36).ok_or(ParseBase36Error::Overflow)?;
        acc = acc.checked_add(v).ok_or(ParseBase36Error::Overflow)?;
    }
    Ok(acc)
}

pub fn to_string(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut buf = [0u8; RANK_LENGTH];
    let mut i = buf.len();

    while n > 0 {
        let d = (n % 36) as u8;
        n /= 36;
        i -= 1;
        buf[i] = match d {
            0..=9 => b'0' + d,
            _ => b'a' + (d - 10),
        };
    }
    String::from_utf8(buf[i..].to_vec()).unwrap()
}

pub fn to_string_padded(n: u64, width: usize) -> String {
    let s = to_string(n);
    if s.len() >= width {
        s
    } else {
        let mut out = String::with_capacity(width);
        for _ in 0..(width - s.len()) {
            out.push('0');
        }
        out.push_str(&s);
        out
    }
}
