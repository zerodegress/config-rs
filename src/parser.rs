use std::ops::{Range, RangeFrom, RangeTo};

use nom::{
    error::{ErrorKind, ParseError},
    AsChar, Compare, CompareResult, FindToken, IResult, InputIter, InputLength, Slice,
};

pub fn anystr(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}

pub fn without_chars<T, E: ParseError<T>, Q>(chars: Q) -> impl FnMut(T) -> IResult<T, T, E>
where
    T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    T: InputIter + InputLength,
    <T as InputIter>::Item: AsChar,
    Q: FindToken<<T as InputIter>::Item>,
{
    move |input: T| match input.position(|item| chars.find_token(item)) {
        None => Ok((input.slice(input.input_len()..), input)),
        Some(index) => Ok((input.slice(index..), input.slice(..index))),
    }
}

pub fn without_chars_and_line_ending<T, E: ParseError<T>, Q>(
    chars: Q,
) -> impl FnMut(T) -> IResult<T, T, E>
where
    T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    T: InputIter + InputLength,
    T: Compare<&'static str>,
    <T as InputIter>::Item: AsChar + Clone,
    Q: FindToken<<T as InputIter>::Item>,
{
    move |input: T| match input.position(|item| {
        item.clone().as_char() == '\r' || item.clone().as_char() == '\n' || chars.find_token(item)
    }) {
        None => Ok((input.slice(input.input_len()..), input)),
        Some(index) => {
            let mut it = input.slice(index..).iter_elements();
            let nth = it.next().unwrap().as_char();
            if nth == '\r' {
                let sliced = input.slice(index..);
                let comp = sliced.compare("\r\n");
                match comp {
                    //FIXME: calculate the right index
                    CompareResult::Ok => Ok((input.slice(index..), input.slice(..index))),
                    _ => {
                        let e: ErrorKind = ErrorKind::Tag;
                        Err(nom::Err::Error(E::from_error_kind(input, e)))
                    }
                }
            } else {
                Ok((input.slice(index..), input.slice(..index)))
            }
        }
    }
}
