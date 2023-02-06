use nom::bytes::complete::take;
use nom::combinator::flat_map;
use nom::error::ErrorKind;
use nom::number::complete::u8;
use nom::Err;

fn main() {
    let mut parse = flat_map(u8, take);

    let inp1: &[u8] = &[2, 0, 1, 2];
    let (num, res) = parse(inp1).unwrap();

    assert_eq!([2], num);
    assert_eq!([0, 1], res);

    // NOTE: Need the [..]s in Ok(...) below to create a whole array slice
    assert_eq!(parse(&[2, 0, 1, 2]), Ok((&[2][..], &[0, 1][..])));

    assert_eq!(
        parse(&[4, 0, 1, 2]),
        Err(Err::Error((&[0, 1, 2][..], ErrorKind::Eof)))
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn two_arrays() {
        let a1 = [1, 2, 3];
        let a2 = [1, 2, 3];

        assert_eq!(a1, a2);
    }

    fn an_ok(ary: &[u8]) -> Result<&[u8], &str> {
        if ary.len() > 1 {
            // Ok(&ary[0..1])
            Ok(&ary[0..1])
        } else {
            Err("Too short!")
        }
    }

    #[test]
    fn a_good_ary() {
        let expected = &[1u8];
        let result = an_ok(&[1u8, 2u8]).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn same_good_ary() {
        let expected = [1u8];
        let expected_slice = &expected[..];

        assert_eq!(Ok(expected_slice), an_ok(&[1u8, 3u8]));
    }
}
