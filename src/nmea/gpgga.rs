use std::fmt::Display;

use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct NmeaGpggaS {
	base: NmeaS,
	time: Tm,
	longitude: NmeaPosition,
	latitude: NmeaPosition,
	n_satellites: i32,
	altitude: f64,
	altitude_unit: c_char,
	undulation: f64,
	undulation_unit: c_char,
	position_fix: u8,
}

impl Display for NmeaGpggaS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPGGA sentence")?;
        writeln!(f, "Number of satellites: {}", self.n_satellites)?;
        write!(f, "Altitude: {:.6} M", self.altitude)
    }
}