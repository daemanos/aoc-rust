use std::convert::TryFrom;
use std::iter::Peekable;
use std::str::Chars;

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
