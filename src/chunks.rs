use nom::Endianness;
use nom::*;
use sample::*;

#[derive(Debug)]
pub struct ChunkHeader<'a> {
    id: &'a str,
    size:u32,
}
#[derive(Debug)]
pub struct MasterChunk<'a> {
    pub header: ChunkHeader<'a>,
    pub format: &'a str
}
#[derive(Debug)]
pub struct FmtChunk<'a> {
    pub header : ChunkHeader<'a>,
    audio_format: u16,
    num_channels: u16,
    sample_rate : u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16
}

#[derive(Debug)]
pub struct DataChunk<'a> {
    header: ChunkHeader<'a>,
    channels: Channels //sample set of multi track
}


#[derive(Debug)]
pub struct WAVE<'a> {
    pub master: MasterChunk<'a>,
    pub fmt : FmtChunk<'a>,
    pub data : DataChunk<'a>
}


named!(chunk_header<&[u8],ChunkHeader>,
    do_parse!(
        id: take_str!(4) >>
        size: u32!(Endianness::Little) >>
        (ChunkHeader{id,size})
    )
);

named!(master_chunk<&[u8],MasterChunk>,
    do_parse!(
       header : chunk_header >>
       format : take_str!(4) >>
       (MasterChunk{header,format})
    )
);

named!(fmt_chunk<&[u8],FmtChunk>,
    do_parse!(
        header: chunk_header >>
        audio_format: u16!(Endianness::Little) >>
        num_channels: u16!(Endianness::Little) >>
        sample_rate: u32!(Endianness::Little) >>
        byte_rate: u32!(Endianness::Little) >>
        block_align: u16!(Endianness::Little) >>
        bits_per_sample: u16!(Endianness::Little) >>
        take_str!(header.size - (2+2+4+4+2+2)) >>
        (FmtChunk{header,audio_format,num_channels,sample_rate,byte_rate,block_align,bits_per_sample})
    )
);



fn data_chunk<'a>(input:&'a[u8],fmt:&FmtChunk) -> IResult<&'a[u8],DataChunk<'a>> {
    let h:ChunkHeader;
    let c:Channels;
    let mut next = input;

    {
        let ret = chunk_header(next);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        }else{
            let (i,t) = ret.unwrap();
            next = i;
            h = t;
        }
    }

    println!("{:?}",h);

    {
        let ret = channels(next,h.size as usize,fmt.num_channels as u32,fmt.bits_per_sample as u32);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        }else{
            let (i,t) = ret.unwrap();
            next = i;
            c = t;
        }
    }
    Ok((next,DataChunk{header:h,channels:c}))
}

named!(pub parse_wave<&[u8],WAVE>,
    do_parse!(
        master: master_chunk >>
        fmt : fmt_chunk >>
        data : call!(data_chunk,&fmt) >>
        (WAVE{master:master,fmt:fmt,data:data})
    )
);

#[test]
fn test_header() {
   let f = include_bytes!("../samples/a2002011001-e02.wav");

    let ret = parse_wave(f);
    let (i,mut instance) = ret.unwrap();

    assert_eq!(instance.master.header.id,"RIFF");
    assert_eq!(instance.master.format,"WAVE");
    assert_eq!(instance.fmt.header.id,"fmt ");
    assert_eq!(instance.fmt.header.size,18);
    assert_eq!(instance.fmt.audio_format,1);
    assert_eq!(instance.fmt.num_channels,2);
    assert_eq!(instance.fmt.sample_rate,44100);
    assert_eq!(instance.fmt.byte_rate,176400);
    assert_eq!(instance.fmt.block_align,4);
    assert_eq!(instance.fmt.bits_per_sample,16);
}
