pub struct Image8BPP<'a> {
	pub width: u32,
	pub height: u32,
	pub imagedata: &'a [u8],
}

pub struct Image1BPP<'a> {
	pub width: u32,
	pub height: u32,
	pub imagedata: &'a [u8],
}