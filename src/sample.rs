use nom;
use nom::*;
use nom::IResult;
use error;
use macros;

pub enum Sample {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

pub type SampleSet = Vec<Sample>;

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


