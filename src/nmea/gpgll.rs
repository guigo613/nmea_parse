use std::fmt::Display;

use super::{NmeaPosition, NmeaS, Tm};

#[repr(C)]
#[derive(Debug)]
pub struct NmeaGpgllS {
	base: NmeaS,
	longitude: NmeaPosition,
	latitude: NmeaPosition,
	time: Tm
}

impl Display for NmeaGpgllS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPGLL sentence")?;
        writeln!(f, "Longitude:")?;
        writeln!(f, "{:>2}", self.longitude)?;
        writeln!(f, "Latitude:")?;
        writeln!(f, "{:>2}", self.latitude)?;
        write!(f, "Time: {:02}:{:02}:{:02}", self.time.tm_hour, self.time.tm_min, self.time.tm_sec)
    }
}