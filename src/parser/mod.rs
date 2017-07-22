//! Parser module is responsible for parsing FITS files.

named!(fits<&[u8], (Vec<&[u8]>, Vec<&[u8]>) >,
       pair!(primary_header, many0!( take!(2880) )));

named!(primary_header<&[u8], Vec<&[u8]> >, count!(take!(2880), 2));

named!(end_header<&[u8],(&[u8], Vec<&[u8]>)>, pair!(tag!("END"), count!(tag!(" "), 77)));

named!(blank_header<&[u8],Vec<&[u8]> >, count!(tag!(" "), 80));

#[cfg(test)]
mod tests {
    use nom::{IResult};
    use super::{fits, end_header, blank_header};

    #[test]
    fn it_should_parse_a_fits_file(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = fits(data);

        match result {
            IResult::Done(_, (header, blocks)) => {
                assert_eq!(header.len(), 2);
                assert_eq!(blocks.len(), 3675);
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn end_header_should_parse_an_END_header(){
        let data = "END                                                                             "
            .as_bytes();

        let result = end_header(data);

        match result {
            IResult::Done(_,_) => assert!(true),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn blank_header_should_parse_a_BLANK_header(){
        let data = "                                                                                "
            .as_bytes();

        let result = blank_header(data);

        match result {
            IResult::Done(_,_) => assert!(true),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }
}
