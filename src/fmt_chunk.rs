use chunk_header::chunk_header;
use nom::Endianness;
use nom::*;
pub struct FmtChunk<'a> {
    pub header : ::ChunkHeader<'a>,
    audio_format: u16,
    num_channels: u16,
    sample_rate : u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16
}

named!(pub fmt_chunk<&[u8],FmtChunk>,
    do_parse!(
        header: chunk_header >>
        audio_format: u16!(Endianness::Little) >>
        num_channels: u16!(Endianness::Little) >>
        sample_rate: u32!(Endianness::Little) >>
        byte_rate: u32!(Endianness::Little) >>
        block_align: u16!(Endianness::Little) >>
        bits_per_sample: u16!(Endianness::Little) >>
        (FmtChunk{header,audio_format,num_channels,sample_rate,byte_rate,block_align,bits_per_sample})
    )
);



