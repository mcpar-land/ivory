use nom::IResult;

pub trait Parse: Sized + std::fmt::Debug + Clone {
	fn parse(input: &str) -> IResult<&str, Self>;
}

impl Parse for () {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}
