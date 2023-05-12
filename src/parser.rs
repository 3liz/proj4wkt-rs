//!
//! Parser/Tokenizer
//!
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{all_consuming, cut, fail, iterator, map, recognize},
    multi::{fold_many0, many0_count},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use crate::errors::{Error, Result};
use crate::log;

/// Literal attribute
#[derive(Debug, PartialEq)]
pub enum Attribute<'a, T> {
    Quoted(&'a str),
    Number(&'a str),
    Label(&'a str),
    Keyword(&'a str, T),
}

impl<'a, T> Attribute<'a, T> {
    pub fn as_str(&self) -> &'a str {
        match self {
            Self::Quoted(s) | Self::Number(s) | Self::Keyword(s, _) | Self::Label(s) => s,
        }
    }
}

pub trait Processor<'a> {
    type Err;
    type Output;

    fn process<I>(&self, key: &'a str, depth: usize, attrs: I) -> Result<Self::Output, Self::Err>
    where
        I: Iterator<Item = Attribute<'a, Self::Output>>;
}

pub fn parse<'a, P, O>(i: &'a str, p: &P) -> Result<O>
where
    P: Processor<'a, Output = O>,
{
    all_consuming(|i: &'a str| object(i, p, 0))(i)
        .map_err(|_| Error::ParseError)
        .map(|(_, value)| match value {
            Attribute::Keyword(_, out) => out,
            _ => unreachable!(),
        })
}

// Single quote delimited string
fn quoted_string<'a>(i: &'a str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        |s: &'a str| {
            map(
                fold_many0(
                    alt((tag("\"\""), is_not("\""))),
                    || 0,
                    |n, item: &str| n + item.len(),
                ),
                |len| &s[..len],
            )(s)
        },
        char('"'),
    )(i)
}

fn number(i: &str) -> IResult<&str, &str> {
    alt((recognize_float, recognize(digit1)))(i)
}

fn keyword(i: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(i)
}

fn log_failure<E, T>(_err: E) -> IResult<&'static str, T> {
    log::error!("Wkt failure {_err:?}");
    cut(fail)("")
}

// Process object attribute
fn object<'a, P, O>(i: &'a str, p: &P, depth: usize) -> IResult<&'a str, Attribute<'a, O>>
where
    P: Processor<'a, Output = O>,
{
    terminated(keyword, trim_left(char('[')))(i.trim_start()).and_then(|(rest, key)| {
        attribute_list(rest, p, depth, key).and_then(|(rest, node)| {
            match cut(trim_left(char(']')))(rest) {
                Ok((rest, _)) => Ok((rest, node)),
                Err(err) => {
                    log::error!("Missing closing delimiter for {kw}");
                    Err(err)
                }
            }
        })
    })
}

// Parse attributes list
// attribute_list:
//      attribute_list COMMA attribute
//    | attribute
fn attribute_list<'a, P, O>(
    i: &'a str,
    p: &P,
    depth: usize,
    key: &'a str,
) -> IResult<&'a str, Attribute<'a, O>>
where
    P: Processor<'a, Output = O>,
{
    let (rest, attr) = attribute(i, p, depth)?;

    let mut it = iterator(
        rest,
        preceded(trim_left(char(',')), |i: &'a str| attribute(i, p, depth)),
    );

    match p.process(key, depth, std::iter::once(attr).chain(&mut it)) {
        Ok(node) => {
            let (rest, _) = it.finish()?;
            Ok((rest, Attribute::Keyword(key, node)))
        }
        Err(err) => log_failure(err),
    }
}

// Parse Node
// attribute:
//     keyword attibute_list
//   | quoted_string
//   | number
fn attribute<'a, P, O>(i: &'a str, p: &P, depth: usize) -> IResult<&'a str, Attribute<'a, O>>
where
    P: Processor<'a, Output = O>,
{
    let i = i.trim_start();
    object(i, p, depth + 1)
        .or_else(|_| map(quoted_string, |s| Attribute::Quoted(s))(i))
        .or_else(|_| map(number, |n| Attribute::Number(n))(i))
        .or_else(|_| map(keyword, |l| Attribute::Label(l))(i))
}

// Trim whitespaces
#[inline]
pub(super) fn trim_left<
    'a,
    O,
    E: nom::error::ParseError<&'a str>,
    F: nom::Parser<&'a str, O, E>,
>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&str, O, E> {
    preceded(multispace0, f)
}

// ==============================
//  Tests
// ==============================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{Error, Result};

    #[test]
    fn parse_quoted_str() {
        assert_eq!(quoted_string(r#""""#), Ok(("", "")));
        assert_eq!(quoted_string(r#""foo""bar""#), Ok(("", r#"foo""bar"#)));
        assert_eq!(quoted_string(r#""foobar" baz"#), Ok((" baz", r#"foobar"#)));
    }

    #[test]
    fn parse_number() {
        assert_eq!(number("1234.56"), Ok(("", "1234.56")));
        assert_eq!(number("1234"), Ok(("", "1234")));
        assert!(number("baz").is_err());
    }

    #[test]
    fn parse_keyword() {
        assert_eq!(keyword("KEY"), Ok(("", "KEY")));
        assert_eq!(keyword("KEY12_"), Ok(("", "KEY12_")));
        assert_eq!(keyword("_KEY1"), Ok(("", "_KEY1")));
        assert_eq!(keyword("_1KEY"), Ok(("", "_1KEY")));
        assert!(keyword("1KEY").is_err());
    }

    #[derive(Debug, PartialEq)]
    struct Node<'a>(&'a str, Vec<Attribute<'a, Node<'a>>>);

    #[derive(Default)]
    struct Builder;

    impl<'a> Processor<'a> for Builder {
        type Err = Error;
        type Output = Node<'a>;

        fn process<I>(
            &self,
            key: &'a str,
            depth: usize,
            attrs: I,
        ) -> Result<Self::Output, Self::Err>
        where
            I: Iterator<Item = Attribute<'a, Self::Output>>,
        {
            Ok(Node(key, attrs.collect()))
        }
    }

    #[test]
    fn parse_wkt() {
        let wkt = parse(r#"FOO["foo", BAR["bar"], baz]"#, &Builder::default()).unwrap();

        assert_eq!(
            wkt,
            Node(
                "FOO",
                vec![
                    Attribute::Quoted("foo"),
                    Attribute::Keyword("BAR", Node("BAR", vec![Attribute::Quoted("bar")])),
                    Attribute::Label("baz"),
                ],
            ),
        );
    }

    #[test]
    fn parse_failure() {
        let mut wkt = Builder::default();
        assert!(parse(r#"FOO["foo", BAR["bar"]"#, &mut wkt).is_err());
    }
}
