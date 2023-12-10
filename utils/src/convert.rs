use std::convert::TryFrom;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use core::str::pattern::Pattern;

pub trait PeekFrom<I>: Sized
where I: Iterator
{
    fn peek_from(iter: &mut Peekable<I>) -> Option<Self>;
}

impl<'a, T> PeekFrom<Chars<'a>> for T
where T: TryFrom<char> {
    fn peek_from(iter: &mut Peekable<Chars<'a>>) -> Option<Self> {
        iter.next().and_then(|ch| T::try_from(ch).ok())
    }
}

/// Splits a string into words separated by whitespace and converts each word
/// to the given type.
///
/// # Examples
/// ```
/// # use utils::convert::words;
/// let v: Vec<u32> = words("1 23 456").collect();
/// assert_eq!(vec![1, 23, 456], v);
/// ```
pub fn words<T>(s: &str) -> impl Iterator<Item = T> + '_
where T: FromStr,
{
    s.split_whitespace().filter_map(|word| word.parse().ok())
}

/// Splits a string into parts delimited by the given pattern and converts each
/// part to the given type.
///
/// # Examples
/// ```
/// # use utils::convert::delimited;
/// let v: Vec<u32> = delimited("1:23:456", ':').collect();
/// assert_eq!(vec![1, 23, 456], v);
/// ```
pub fn delimited<'a, T, P>(s: &'a str, pat: P) -> impl Iterator<Item = T> + 'a
where T: FromStr,
      P: Pattern<'a> + 'a
{
    s.split(pat).filter_map(|word| word.parse().ok())
}
