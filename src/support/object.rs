use gl::types::{GLsizeiptr, GLuint};
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
use std::{mem, ptr, str};

const HEADER_TAG: &str = "SB6M";
const INDEX_DATA_TAG: &str = "INDX";
const VERTEX_DATA_TAG: &str = "VRTX";
const VERTEX_ATTRIBUTES_TAG: &str = "ATRB";
const SUB_OBJECT_LIST_TAG: &str = "OLST";
const COMMENT_TAG: &str = "CMNT";
const DATA_TAG: &str = "DATA";

const CHUNK_HEADER_BYTES: u32 = 4;
const VERTEX_ATTRIBUTE_NAME_BYTES: u32 = 64;

const VERTEX_ATTRIB_FLAG_NORMALIZED: u32 = 0x0000_0001;
// const VERTEX_ATTRIB_FLAG_INTEGER: u32 = 0x0000_0002;

#[derive(Debug)]
pub enum ChunkType<'a> {
    IndexData(IndexData),
    VertexData(VertexData<'a>),
    VertexAttributes(Vec<VertexAttribute>),
    SubObjects(Vec<SubObject>),
    Comment(String),
    Data(Data),
}

#[derive(Debug)]
pub struct VertexData<'a> {
    data_size: isize,
    data_offset: u32,
    total_vertices: u32,
    vertices: &'a [u8],
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

impl VertexAttribute {
    fn is_normalized(&self) -> u8 {
        if self.flags & VERTEX_ATTRIB_FLAG_NORMALIZED == 1 {
            gl::TRUE
        } else {
            gl::FALSE
        }
    }
}

#[derive(Debug, Default)]
pub struct SubObject {
    first: u32,
    count: u32,
}

#[derive(Debug, Default)]
pub struct Object {
    vbo: GLuint,
    vao: GLuint,
    sub_objects: Vec<SubObject>,
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
fn chunk<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(input: &'static [u8]) -> IResult<&'static [u8], ChunkType, E> {
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

fn chunk_header<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], ChunkHeader, E> {
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

fn index_data<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], IndexData, E> {
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

fn comment<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], String, E> {
    let (input, header) = chunk_header(input)?;
    context(
        "Comment",
        cut(map(
            take((header.chunk_size - 8_u32) as usize),
            bytes_to_string,
        )),
    )(input)
}

fn data<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], Data, E> {
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

fn vertex_data<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], VertexData, E> {
    let (input, _) = chunk_header(input)?;
    let (input, data_size) = le_u32(input)?;
    context(
        "Vertex Data",
        cut(map(
            tuple((le_u32, le_u32, take(data_size as usize))),
            |(data_offset, total_vertices, vertices): (u32, u32, &[u8])| VertexData {
                data_size: (vertices.len() * mem::size_of::<u8>()) as GLsizeiptr,
                data_offset,
                total_vertices,
                vertices,
            },
        )),
    )(input)
}

fn vertex_attributes<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], Vec<VertexAttribute>, E> {
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

fn sub_objects<E: ParseError<&'static [u8]> + nom::error::ContextError<&'static [u8]>>(
    input: &'static [u8],
) -> IResult<&'static [u8], Vec<SubObject>, E> {
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

pub fn parse_object(input: &'static [u8]) -> IResult<&'static [u8], Object> {
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

    let object = prepare_object(chunks);

    Ok((input, object))
}

fn prepare_object(chunks: Vec<ChunkType>) -> Object {
    // TODO:
    // Only one of the book's models (asteroids.sbm) use chunks besides the VertexData and VertexAttributes chunks.
    // So this doesn't use any info from the chunks other than that.
    // If a .sbm file comes up that actually has IndexData in it, this will have to be updated
    // according to the source from the book's github repo.
    // The IndexData and Data chunks will need to have data read in as well.

    let mut _comment_chunk: Option<String> = None;
    let mut _data_chunk: Option<Data> = None;
    let mut _index_data_chunk: Option<IndexData> = None;
    let mut sub_objects_chunk: Option<Vec<SubObject>> = None;
    let mut vertex_attributes_chunk: Option<Vec<VertexAttribute>> = None;
    let mut vertex_data_chunk: Option<VertexData> = None;

    for chunk in chunks {
        match chunk {
            ChunkType::Comment(comment) => _comment_chunk = Some(comment),
            ChunkType::Data(data) => _data_chunk = Some(data),
            ChunkType::IndexData(index_data) => _index_data_chunk = Some(index_data),
            ChunkType::SubObjects(sub_objects) => sub_objects_chunk = Some(sub_objects),
            ChunkType::VertexAttributes(attributes) => vertex_attributes_chunk = Some(attributes),
            ChunkType::VertexData(vertex_data) => vertex_data_chunk = Some(vertex_data),
        }
    }

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let mut data_size = 0;

    if vertex_data_chunk.is_some() {
        let vertex_data = vertex_data_chunk.as_ref().unwrap();
        data_size += vertex_data.data_size;
    }

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, data_size, ptr::null(), gl::STATIC_DRAW);
    }

    if vertex_data_chunk.is_some() {
        let vertex_data = vertex_data_chunk.as_ref().unwrap();
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                vertex_data.data_size,
                vertex_data.vertices.as_ptr() as *const gl::types::GLvoid,
            );
        }
    }

    let vertex_attributes = vertex_attributes_chunk.as_ref().unwrap();
    for (index, attribute) in vertex_attributes.iter().enumerate() {
        unsafe {
            gl::VertexAttribPointer(
                index as u32,
                attribute.size as i32,
                attribute.attribute_type,
                attribute.is_normalized(),
                attribute.stride as i32,
                attribute.data_offset as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(index as u32);
        }
    }

    unsafe {
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    let sub_objects = if sub_objects_chunk.is_none() {
        let mut sub_object_list = Vec::new();
        let vertex_data = vertex_data_chunk.as_ref().unwrap();
        sub_object_list.push(SubObject {
            first: 0,
            count: vertex_data.total_vertices,
        });
        sub_object_list
    } else {
        sub_objects_chunk.unwrap()
    };

    Object {
        vbo,
        vao,
        sub_objects,
    }
}

pub fn render_all(object: &Object) {
    for (index, _) in object.sub_objects.iter().enumerate() {
        render_object(object, index as u32, 1, 0);
    }
}

pub fn render_object(object: &Object, index: u32, instance_count: u32, base_instance: u32) {
    unsafe {
        gl::BindVertexArray(object.vao);
        gl::DrawArraysInstancedBaseInstance(
            gl::TRIANGLES,
            object.sub_objects[index as usize].first as i32,
            object.sub_objects[index as usize].count as i32,
            instance_count as i32,
            base_instance,
        );
    }
}
