use nom;
use nom::*;
use nom::IResult;
use error;
use macros;
use std::vec;

#[derive(Debug)]
pub enum Sample {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

pub type SampleSet = Vec<Sample>;

pub type Channels = Vec<SampleSet>;

pub fn sample(input:&[u8],size:u32) -> IResult<&[u8],Sample> {

  match size {
        8 => {
            le_u8(input).map(|(input,s)|{(input,Sample::U8(s))})
        },
        16 => {
            le_u16(input).map(|(input,s)|{(input,Sample::U16(s))})
        },
        32 => {
            le_u32(input).map(|(input,s)|{(input,Sample::U32(s))})
        }
        64 => {
            le_u64(input).map(|(input,s)|{(input,Sample::U64(s))})
        }
        _ => {
            custom_error!(input,error::UNSUPPORTED_BITS)
        }
    }
}

pub fn channels(input:&[u8],len:usize,num_channels:u32,size:u32) -> IResult<&[u8],Channels> {
    let mut channels:Channels = vec::Vec::with_capacity(num_channels as usize);
    for _ in 0..num_channels {
        channels.push(SampleSet::new());
    }
    let mut next:&[u8] = input;
    let mut temp_sample:Sample;
    let mut used = 0;
    while used < len - size as usize {
        for c in 0..num_channels {
            let ret = sample(next, size);
            if ret.is_err() {
                return Err(ret.err().unwrap());
            }
            let (n, t) = ret.unwrap();
            next = n;
            temp_sample = t;
            channels[c as usize].push(temp_sample);
            used += (size as usize);
        }
    }
    Ok((next,channels))
}


