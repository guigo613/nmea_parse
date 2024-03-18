use std::{ffi::c_char, fmt::Display};

use crate::nmea::{NMEA_CARDINAL_DIR_EAST, NMEA_CARDINAL_DIR_WEST};

use super::{NmeaPosition, NmeaS, Tm};



#[repr(C)]
#[derive(Debug)]
pub struct NmeaGprmcS {
	base: NmeaS,
	date_time: Tm,
	longitude: NmeaPosition,
	latitude: NmeaPosition,
	gndspd_knots: f64,
	track_deg: f64,
	magvar_deg: f64,
	magvar_cardinal: c_char,
	//The direction of the magnetic variation determines whether or not it
	//is additive - Easterly means subtract magvar_deg from track_deg and
	//westerly means add magvar_deg to track_deg for the correct course.
	valid: bool
}

impl Display for NmeaGprmcS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPRMC sentence")?;
        writeln!(f, "Longitude:")?;
        writeln!(f, "{:>2}", self.longitude)?;
        writeln!(f, "Latitude:")?;
        writeln!(f, "{:>2}", self.latitude)?;
        writeln!(f, "Date & Time: {}", self.date_time)?;
        writeln!(f, "Speed, in Knots: {:.6}:", self.gndspd_knots)?;
        writeln!(f, "Track, in degrees: {:.6}", self.track_deg)?;
        writeln!(f, "Magnetic Variation:")?;
        writeln!(f, "  Degrees: {:.6}", self.magvar_deg)?;
        writeln!(f, "  Cardinal: {}", self.magvar_cardinal as u8 as char)?;

        let mut adjusted_course = self.track_deg;
        
        if (NMEA_CARDINAL_DIR_EAST as i8 == self.magvar_cardinal) {
            adjusted_course -= self.magvar_deg;
        } else if (NMEA_CARDINAL_DIR_WEST  as i8 == self.magvar_cardinal) {
            adjusted_course += self.magvar_deg;
        } else {
            writeln!(f, "Invalid Magnetic Variation Direction!!");
        }
        
        write!(f, "Adjusted Track (heading): {adjusted_course:.6}")
    }
}