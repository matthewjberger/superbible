use gl::types::GLuint;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::rest,
    multi::count,
    number::complete::{be_u32, be_u8, le_u32, le_u8},
    sequence::tuple,
    IResult,
};
use std::str;

const HEADER_TAG: &str = "SB6M";
const INDEX_DATA_TAG: &str = "INDX";
const VERTEX_DATA_TAG: &str = "VRTX";
const VERTEX_ATTRIBS_TAG: &str = "ATRB";
const SUB_OBJECT_LIST_TAG: &str = "OLST";
const COMMENT_TAG: &str = "CMNT";
const DATA_TAG: &str = "DATA";

const VERTEX_ATTRIB_FLAG_NORMALIZED: u32 = 0x0000_0001;
const VERTEX_ATTRIB_FLAG_INTEGER: u32 = 0x0000_0002;

pub struct VertexAttribDecl {
    name: [u8; 64],
    size: u32,
    r#type: u32,
    stride: u32,
    flags: u32,
    data_offset: u32,
}

#[derive(Default, Copy, Clone)]
pub struct SubObjectDecl {
    first: u32,
    count: u32,
}

pub struct Object {
    data_buffer: GLuint,
    vao: GLuint,
    index_type: GLuint,
    index_offset: GLuint,
    sub_object: [SubObjectDecl; 256],
}

#[macro_export]
macro_rules! load_object {
    ($path:tt) => {
        $crate::object::parse_object(include_bytes!($path))
    };
}

pub fn parse_object<'a>(input: &'a [u8]) -> IResult<&'a [u8], Object> {
    let mut sub_object = [SubObjectDecl { first: 0, count: 0 }; 256];

    let (input, _) = alt((
        tag(HEADER_TAG),                                              // Little Endian
        tag(HEADER_TAG.chars().rev().collect::<String>().as_bytes()), // Big Endian
    ))(input)?;

    // No header flags are defined for the format currently so they are ignored
    let (input, (size, num_chunks, _)) = tuple((le_u32, le_u32, le_u32))(input)?;

    // Data can be stored between header and first chunk,
    // so advance to first chunk by advancing the size
    // of the header (16 bytes) minus the current position in the file
    let (input, _) = take((size - 16) as usize)(input)?;

    for _ in 0..num_chunks {
        let (input, chunk_type) = alt((
            tag(INDEX_DATA_TAG),
            tag(VERTEX_DATA_TAG),
            tag(VERTEX_ATTRIBS_TAG),
            tag(SUB_OBJECT_LIST_TAG),
            tag(COMMENT_TAG),
            tag(DATA_TAG),
        ))(input)?;

        let chunk_type = str::from_utf8(&chunk_type).unwrap();

        match chunk_type {
            INDEX_DATA_TAG => {
                let (input, (index_type, index_count, index_data_offset)) =
                    tuple((le_u32, le_u32, le_u32))(input)?;
            }
            VERTEX_DATA_TAG => {
                let (input, (data_size, offset, total_vertices)) =
                    tuple((le_u32, le_u32, le_u32))(input)?;
            }
            VERTEX_ATTRIBS_TAG => {
                let (input, attrib_count) = le_u32(input)?;
                println!("---");
                // TODO: Maybe use the 'many' combinator here instead
                for _ in 0..attrib_count {
                    // TODO: Load the vertex attrib decl here
                    let (input, (name, size, attrib_type, stride, flags, data_offset)) =
                        tuple((take(64_usize), le_u32, le_u32, le_u32, le_u32, le_u32))(input)?;
                    let name = str::from_utf8(&name).unwrap();
                    println!("NAME: {}", name);
                    println!("SIZE: {}", size);
                    println!("ATTRIB_TYPE: {}", attrib_type);
                    println!("STRIDE: {}", stride);
                    println!("FLAGS: {}", flags);
                    println!("DATA_OFFSET: {}", data_offset);
                    println!("---");

                    // TODO: input isn't being updated
                }
            }
            SUB_OBJECT_LIST_TAG => {
                let (input, count) = le_u32(input)?;
                // TODO: Load the chunk sub object list
            }
            COMMENT_TAG => {
                // Comment chunks should be skipped
            }
            DATA_TAG => {
                let (input, (encoding, data_offset, data_length)) =
                    tuple((le_u32, le_u32, le_u32))(input)?;
            }
            _ => {}
        };
    }

    Ok((
        input,
        Object {
            data_buffer: 0,
            vao: 0,
            index_type: 0,
            index_offset: 0,
            sub_object: sub_object,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_object() {
        assert!(
            !load_object!("../../assets/objects/torus_nrms_tc.sbm").is_err(),
            "Failed to load the object!"
        );
    }
}
