use gl::types::GLuint;
use nom::{
    bytes::complete::{tag, take},
    combinator::rest,
    multi::count,
    number::complete::{be_u32, le_u32},
    IResult,
};

macro_rules! fourcc(
    ($a:expr, $b:expr, $c:expr, $d:expr) => (
        (($a as u32) << 0 | ($b as u32) << 8 | ($c as u32) << 16 | ($d as u32) << 24)
    );
);

const HEADER_TAG: u32 = fourcc!('S', 'B', '6', 'M');
const INDEX_DATA: u32 = fourcc!('I', 'N', 'D', 'X');
const VERTEX_DATA: u32 = fourcc!('V', 'R', 'T', 'X');
const VERTEX_ATTRIBS: u32 = fourcc!('A', 'T', 'R', 'B');
const SUB_OBJECT_LIST: u32 = fourcc!('O', 'L', 'S', 'T');
const COMMENT: u32 = fourcc!('C', 'M', 'N', 'T');
const DATA: u32 = fourcc!('D', 'A', 'T', 'A');

const VERTEX_ATTRIB_FLAG_NORMALIZED: u32 = 0x0000_0001;
const VERTEX_ATTRIB_FLAG_INTEGER: u32 = 0x0000_0002;

enum ChunkType {
    IndexData,
    VertexData,
    VertexAttribs,
    SubObjectList,
    Comment,
    Data,
}

enum DataEncoding {
    RAW,
}

pub struct FileHeader {
    identifier: u32,
    size: u32,
    num_chunks: u32,
    flags: u32,
}

pub struct ChunkHeader {
    chunk_type: u32,
    size: u32,
}

pub struct ChunkIndexData {
    header: ChunkHeader,
    index_type: u32,
    index_count: u32,
    index_data_offset: u32,
}

pub struct ChunkVertexData {
    header: ChunkHeader,
    data_size: u32,
    data_offset: u32,
    total_vertices: u32,
}

pub struct VertexAttribDecl {
    name: [u8; 64],
    size: u32,
    r#type: u32,
    stride: u32,
    flags: u32,
    data_offset: u32,
}

pub struct VertexAttribChunk {
    header: ChunkHeader,
    attrib_count: u32,
    attrib_data: VertexAttribDecl,
}

pub struct DataChunk {
    header: ChunkHeader,
    encoding: u32,
    data_offset: u32,
    data_length: u32,
}

pub struct SubObjectDecl {
    first: u32,
    count: u32,
}

pub struct SubObjectList {
    header: ChunkHeader,
    count: u32,
    sub_object: Vec<SubObjectDecl>,
}

pub struct ChunkComment<'a> {
    header: ChunkHeader,
    comment: &'a str,
}

pub struct Object {
    data_buffer: GLuint,
    vao: GLuint,
    index_type: GLuint,
    index_offset: GLuint,
    sub_object: Vec<SubObjectDecl>,
}

#[macro_export]
macro_rules! load_object {
    ($path:tt) => {
        $crate::object::parse_object(include_bytes!($path))
    };
}

pub fn parse_object<'a>(input: &'a [u8]) -> IResult<&'a [u8], Object> {
    unimplemented!()
    // Ok(
    //     input,
    //     Object {
    //         data_buffer: 0,
    //         vao: 0,
    //         index_type: 0,
    //         index_offset: 0,
    //         sub_object: Vec::new(),
    //     },
    // )
}
