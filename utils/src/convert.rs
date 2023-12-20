use std::convert::TryFrom;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::str::pattern::{Pattern, ReverseSearcher};

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

pub trait AocString {
    /// Splits a string into words separated by whitespace and converts each
    /// word to the given type.
    fn words<T: FromStr>(&self) -> impl Iterator<Item = T>;

    /// Splits a string into parts delimited by the given pattern and converts
    /// each part to the given type.
    fn split_into<'a, T, P>(&'a self, pat: P) -> impl Iterator<Item = T>
        where T: FromStr,
              P: Pattern<'a>;

    fn strip_circumfix<'a, P>(&'a self, pre: P, post: P) -> Option<&'a Self>
        where P: Pattern<'a>,
              <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>;

    fn strip_braces(&self) -> Option<&Self> {
        self.strip_circumfix('{', '}')
    }

    fn strip_brackets(&self) -> Option<&Self> {
        self.strip_circumfix('[', ']')
    }

    fn strip_parens(&self) -> Option<&Self> {
        self.strip_circumfix('(', ')')
    }
}

impl AocString for str {
    fn words<T: FromStr>(&self) -> impl Iterator<Item = T> {
        self.split_whitespace().filter_map(|word| word.parse().ok())
    }

    fn split_into<'a, T, P>(&'a self, pat: P) -> impl Iterator<Item = T>
    where T: FromStr,
          P: Pattern<'a>,
    {
        self.split(pat).filter_map(|word| word.parse().ok())
    }

    fn strip_circumfix<'a, P>(&'a self, pre: P, post: P) -> Option<&'a str>
        where P: Pattern<'a>,
              <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
    {
        self.strip_prefix(pre)?
            .strip_suffix(post)
    }
}
