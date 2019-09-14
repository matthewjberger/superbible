use gl::types::GLvoid;
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

pub fn prepare_texture(ktx_texture: &KtxData) -> u32 {
    let ktx = &ktx_texture.header;
    let image = &ktx_texture.pixels;
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexStorage2D(
            gl::TEXTURE_2D,
            ktx.mip_levels as i32,
            ktx.gl_internal_format,
            ktx.pixel_width as i32,
            ktx.pixel_height as i32,
        );

        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        let mut image_ptr = image.as_ptr() as *const GLvoid;
        let mut width = ktx.pixel_width as i32;
        let mut height = ktx.pixel_height as i32;

        for level in 0..ktx.mip_levels {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                level as i32,
                0,
                0,
                width,
                height,
                ktx.gl_format,
                ktx.gl_type,
                image_ptr,
            );

            let stride = calculate_stride(&ktx_texture, width, 1);
            image_ptr = image_ptr.offset(height as isize * stride);
            height >>= 1;
            width >>= 1;
            if height == 0 {
                height = 1;
            }
            if width == 0 {
                width = 1;
            }
        }

        if ktx.mip_levels == 1 {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
    texture
}

fn calculate_stride(ktx_texture: &KtxData, width: i32, pad: usize) -> isize {
    let ktx = &ktx_texture.header;
    let channels = match ktx.gl_base_internal_format {
        gl::RED => 1,
        gl::RG => 2,
        gl::BGR | gl::RGB => 3,
        gl::BGRA | gl::RGBA => 4,
        _ => 0,
    };
    (((ktx.gl_type_size * channels * width as u32) as usize + (pad - 1)) & !(pad - 1)) as isize
}
