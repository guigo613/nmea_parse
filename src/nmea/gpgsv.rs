use std::fmt::Display;

use super::NmeaS;

#[repr(C)]
#[derive(Debug)]
pub struct NmeaGpgsvS {
	base: NmeaS,
	sentences: u32,
	sentence_number: u32,
	satellites: u32,
	sat: [Sat; 4]
}

impl Display for NmeaGpgsvS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPGSV Sentence:")?;
        writeln!(f, "  Num: {}", self.sentences)?;
        writeln!(f, "  ID:  {}", self.sentence_number)?;
        writeln!(f, "  SV:  {}", self.satellites)?;
        writeln!(f, "  #1:  {}", self.sat[0])?;
        writeln!(f, "  #2:  {}", self.sat[1])?;
        writeln!(f, "  #3:  {}", self.sat[2])?;
        write!(f, "  #4:  {}", self.sat[3])
    }
}

//Satellite PRN number
//Elevation, degrees
//Azimuth, degrees
//SNR - higher is better
#[repr(C)]
#[derive(Debug)]
pub struct Sat {
    prn: i32,
    elevation: i32,
    azimuth: i32,
    snr: i32
}

impl Display for Sat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.prn, self.elevation, self.azimuth, self.snr)
    }
}