use nom::{bytes::complete::take_while, error::ParseError, IResult};

pub fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \r\n\t";
    take_while(|c| chars.contains(c))(input)
}
