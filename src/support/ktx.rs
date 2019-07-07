use nom::{
    bytes::complete::{tag, take},
    combinator::rest,
    multi::count,
    number::complete::{be_u32, le_u32},
    IResult,
};

pub const KTX_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

#[derive(Debug)]
pub struct Header<'a> {
    pub endianess: &'a [u8],
    pub gl_type: u32,
    pub gl_type_size: u32,
    pub gl_format: u32,
    pub gl_internal_format: u32,
    pub gl_base_internal_format: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub pixel_depth: u32,
    pub array_elements: u32,
    pub faces: u32,
    pub mip_levels: u32,
    pub key_pair_bytes: u32,
}

pub struct KtxData<'a> {
    pub header: Header<'a>,
    pub pixels: &'a [u8],
}

#[macro_export]
macro_rules! load_ktx {
    ($path:tt) => {
        $crate::ktx::parse_ktx(include_bytes!($path))
    };
}

pub fn parse_ktx<'a>(input: &'a [u8]) -> IResult<&'a [u8], KtxData> {
    let (input, _) = tag(KTX_IDENTIFIER)(input)?;
    let (input, endianess) = take(4usize)(input)?;
    let big_endian = endianess[0] == 0x04;
    let take_endian = |input| -> IResult<&'a [u8], u32> {
        if big_endian {
            be_u32(input)
        } else {
            le_u32(input)
        }
    };
    let (input, members) = count(take_endian, 12usize)(input)?;
    let mut member_iter = members.iter();

    let (input, pixels) = rest(input)?;

    Ok((
        input,
        KtxData {
            header: Header {
                endianess,
                gl_type: *member_iter.next().unwrap(),
                gl_type_size: *member_iter.next().unwrap(),
                gl_format: *member_iter.next().unwrap(),
                gl_internal_format: *member_iter.next().unwrap(),
                gl_base_internal_format: *member_iter.next().unwrap(),
                pixel_width: *member_iter.next().unwrap(),
                pixel_height: *member_iter.next().unwrap(),
                pixel_depth: *member_iter.next().unwrap(),
                array_elements: *member_iter.next().unwrap(),
                faces: *member_iter.next().unwrap(),
                mip_levels: *member_iter.next().unwrap(),
                key_pair_bytes: *member_iter.next().unwrap(),
            },
            pixels,
        },
    ))
}
