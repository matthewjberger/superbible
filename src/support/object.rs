use gl::types::GLuint;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{cut, map, peek},
    error::{context, ParseError},
    multi::many_m_n,
    number::complete::le_u32,
    sequence::{preceded, tuple},
    IResult,
};
use std::str;

const HEADER_TAG: &str = "SB6M";
const INDEX_DATA_TAG: &str = "INDX";
const VERTEX_DATA_TAG: &str = "VRTX";
const VERTEX_ATTRIBUTES_TAG: &str = "ATRB";
const SUB_OBJECT_LIST_TAG: &str = "OLST";
const COMMENT_TAG: &str = "CMNT";
const DATA_TAG: &str = "DATA";

const CHUNK_HEADER_BYTES: u32 = 4;
const VERTEX_ATTRIBUTE_NAME_BYTES: u32 = 64;
const COMMENT_BYTES: u32 = 4;

// const VERTEX_ATTRIB_FLAG_NORMALIZED: u32 = 0x0000_0001;
// const VERTEX_ATTRIB_FLAG_INTEGER: u32 = 0x0000_0002;

#[derive(Debug)]
pub enum ChunkType {
    IndexData(IndexData),
    VertexData(VertexData),
    VertexAttributes(Vec<VertexAttribute>),
    SubObjects(Vec<SubObject>),
    Comment(String),
    Data(Data),
}

#[derive(Debug)]
pub struct VertexData {
    data_size: u32,
    data_offset: u32,
    total_vertices: u32,
}

#[derive(Debug)]
pub struct Data {
    encoding: u32,
    data_offset: u32,
    data_length: u32,
}

#[derive(Debug)]
pub struct ChunkHeader {
    name: String,
    chunk_size: u32,
}

#[derive(Debug)]
pub struct IndexData {
    index_type: u32,
    index_count: u32,
    index_data_offset: u32,
}

#[derive(Debug)]
pub struct VertexAttribute {
    name: String,
    size: u32,
    attribute_type: u32,
    stride: u32,
    flags: u32,
    data_offset: u32,
}

#[derive(Debug)]
pub struct SubObject {
    first: u32,
    count: u32,
}

#[derive(Debug)]
pub struct Object {
    data_buffer: GLuint,
    vao: GLuint,
    index_type: GLuint,
    index_offset: GLuint,
    sub_object: Vec<SubObject>,
}

#[macro_export]
macro_rules! load_object {
    ($path:tt) => {
        $crate::object::parse_object(include_bytes!($path))
    };
}

fn bytes_to_string(bytes: &[u8]) -> String {
    str::from_utf8(&bytes)
        .unwrap()
        .trim_end_matches(char::from(0))
        .to_string()
}

#[rustfmt::skip]
fn chunk<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], ChunkType, E> {
    context(
        "Chunk",
        cut(alt((
            preceded(
                peek(tag(INDEX_DATA_TAG)),
                map(index_data, ChunkType::IndexData)),
            preceded(
                peek(tag(COMMENT_TAG)),
                map(comment, ChunkType::Comment)),
            preceded(
                peek(tag(DATA_TAG)),
                map(data, ChunkType::Data)),
            preceded(
                peek(tag(VERTEX_DATA_TAG)),
                map(vertex_data, ChunkType::VertexData),
            ),
            preceded(
                peek(tag(VERTEX_ATTRIBUTES_TAG)),
                map(vertex_attributes, ChunkType::VertexAttributes),
            ),
            preceded(
                peek(tag(SUB_OBJECT_LIST_TAG)),
                map(sub_objects, ChunkType::SubObjects),
            ),
        ))),
    )(input)
}

fn chunk_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], ChunkHeader, E> {
    context(
        "Chunk Header",
        cut(map(
            tuple((take(CHUNK_HEADER_BYTES), le_u32)),
            |(name_bytes, chunk_size)| ChunkHeader {
                name: bytes_to_string(name_bytes),
                chunk_size,
            },
        )),
    )(input)
}

fn index_data<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], IndexData, E> {
    let (input, _) = chunk_header(input)?;
    context(
        "IndexData",
        cut(map(
            tuple((le_u32, le_u32, le_u32)),
            |(index_type, index_count, index_data_offset)| IndexData {
                index_type,
                index_count,
                index_data_offset,
            },
        )),
    )(input)
}

fn comment<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    let (input, header) = chunk_header(input)?;
    context(
        "Comment",
        cut(map(
            take((header.chunk_size - COMMENT_BYTES) as usize),
            bytes_to_string,
        )),
    )(input)
}

fn data<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Data, E> {
    let (input, _) = chunk_header(input)?;
    context(
        "Data",
        cut(map(
            tuple((le_u32, le_u32, le_u32)),
            |(encoding, data_offset, data_length)| Data {
                encoding,
                data_offset,
                data_length,
            },
        )),
    )(input)
}

fn vertex_data<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], VertexData, E> {
    let (input, _) = chunk_header(input)?;
    context(
        "Vertex Data",
        cut(map(
            tuple((le_u32, le_u32, le_u32)),
            |(data_size, data_offset, total_vertices)| VertexData {
                data_size,
                data_offset,
                total_vertices,
            },
        )),
    )(input)
}

fn vertex_attributes<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<VertexAttribute>, E> {
    let (input, _) = chunk_header(input)?;
    let (input, num_attributes) = le_u32(input)?;
    context(
        "Vertex Attributes",
        cut(many_m_n(
            num_attributes as usize,
            num_attributes as usize,
            map(
                tuple((
                    take(VERTEX_ATTRIBUTE_NAME_BYTES),
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                )),
                |(name_bytes, size, attribute_type, stride, flags, data_offset)| VertexAttribute {
                    name: bytes_to_string(name_bytes),
                    size,
                    attribute_type,
                    stride,
                    flags,
                    data_offset,
                },
            ),
        )),
    )(input)
}

fn sub_objects<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<SubObject>, E> {
    let (input, _) = chunk_header(input)?;
    let (input, num_objects) = le_u32(input)?;
    context(
        "Sub Objects",
        cut(many_m_n(
            num_objects as usize,
            num_objects as usize,
            map(tuple((le_u32, le_u32)), |(first, count)| SubObject {
                first,
                count,
            }),
        )),
    )(input)
}

pub fn parse_object<'a>(input: &'a [u8]) -> IResult<&'a [u8], Object> {
    // let mut sub_object = [SubObjectDecl { first: 0, count: 0 }; 256];

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

    let (input, chunks) = many_m_n(num_chunks as usize, num_chunks as usize, chunk)(input)?;
    println!("CHUNKS {:#?}", chunks);

    Ok((
        input,
        Object {
            data_buffer: 0,
            vao: 0,
            index_type: 0,
            index_offset: 0,
            sub_object: Vec::new(),
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
