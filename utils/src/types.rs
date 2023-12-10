use std::str::FromStr;

pub struct Ident<const R: u32>(u32);

impl<const R: u32> Ident<R> {
    pub fn new(s: &str) -> Option<Self> {
        s.chars()
            .map(|ch| ch.to_digit(R))
            .rev().zip(0..)
            .fold(Some(0), |acc, (digit, exp)|
                acc.and_then(|n| digit.map(|digit| n + R.pow(exp) * digit)))
            .map(Self)
    }

    //pub fn to_string(&self) -> String {
    //    let mut buf = String::new();

    //}
}

pub struct InvalidIdent;
impl<const R: u32> FromStr for Ident<R> {
    type Err = InvalidIdent;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or(InvalidIdent)
    }
}
