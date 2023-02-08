use nom::bytes::complete::take;
use nom::combinator::flat_map;
use nom::error::ErrorKind;
use nom::number::complete::u8;
use nom::Err;

fn main() {
    let mut parse = flat_map(u8, take);

    let inp1: &[u8] = &[2, 0, 1, 2];
    let (num, res) = parse(inp1).unwrap(); // We get two slices back

    assert_eq!([2], num);
    assert_eq!([0, 1], res);

    let res = parse(inp1);
    println!("{:?}", res);

    // NOTE: Need the [..]s in Ok(...) below to create a whole array slice, rather than a reference to an array
    assert_eq!(parse(&[2, 0, 1, 2]), Ok((&[2][..], &[0, 1][..])));

    // Another way to see this perhaps
    let res = [0, 1, 2, 3];
    let res_0 = &res[2..3];
    let res_1 = &res[0..2];

    assert_eq!(parse(&[2, 0, 1, 2]), Ok((res_0, res_1)));

    assert_eq!(
        parse(&[4, 0, 1, 2]),
        Err(Err::Error((&[0, 1, 2][..], ErrorKind::Eof)))
    );

    // An attempt an an explanation ...
    let ary = [1u8, 2u8, 3u8]; // ary is an array with type [u8; 3] - ie fixed size array of 3 u8s
    let ary_ref = &ary; // this is a reference to ary with type &[u8; 3] - a THIN pointer, size is known from above type

    // this is a slice of the whole of ary with type &[u8] - a FAT pointer which includes a starting point and a size
    //   the size is determined by the range, which below is [..] for all entries
    let ary_all_slice = &ary[..];
    let ary_last_two_slice = &ary[1..3];

    assert_eq!([2, 3], ary_last_two_slice);

    // Debug and place a breakpoint on the following line. Then look at the ary_ values defined above
    println!(
        "Sizes: ary {}, ary_ref {}, ary_all_slice {}:{:?}, ary_last_two_slice {}:{:?}",
        ary.len(),
        ary_ref.len(),
        ary_all_slice.len(),
        ary_all_slice,
        ary_last_two_slice.len(),
        ary_last_two_slice
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
