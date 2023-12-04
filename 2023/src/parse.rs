use nom::{bytes::complete::take_while, IResult};

pub fn sp(input: &str) -> IResult<&str, &str> {
    let chars = " \r\n\t";
    take_while(|c| chars.contains(c))(input)
}
