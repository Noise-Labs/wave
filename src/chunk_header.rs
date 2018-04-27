use nom::Endianness;
use nom::*;
pub struct ChunkHeader<'a> {
    id: &'a str,
    size:u32,
}

named!(pub chunk_header<&[u8],ChunkHeader>,
    do_parse!(
        id: take_str!(4) >>
        size: u32!(Endianness::Little) >>
        (ChunkHeader{id,size})
    )
);


