use chunk_header::chunk_header;

pub struct MasterChunk<'a> {
    pub header: ::ChunkHeader<'a>,
    pub format: &'a str
}

named!(pub master_chunk<&[u8],MasterChunk>,
    do_parse!(
       header : chunk_header >>
       format : take_str!(4) >>
       (MasterChunk{header,format})
    )
);

