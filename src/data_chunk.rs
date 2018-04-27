use chunk_header::chunk_header;
use sample;

pub struct DataChunk<'a> {
    header: ::ChunkHeader<'a>,
    tracks: Vec<sample::SampleSet>  //sample set of multi track
}


