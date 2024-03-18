use std::{ffi::c_char, fmt::Display};

use super::NmeaS;

// M = manual (forced 2D or 3D), A = automatic
// 1 = no fix, 2 = 2D, 3 = 3D
#[repr(C)]
#[derive(Debug)]
pub struct NmeaGpgsaS {
	base: NmeaS,
	mode: c_char,
	fixtype: c_char,
	sat_id_00: i32,
	sat_id_01: i32,
	sat_id_02: i32,
	sat_id_03: i32,
	sat_id_04: i32,
	sat_id_05: i32,
	sat_id_06: i32,
	sat_id_07: i32,
	sat_id_08: i32,
	sat_id_09: i32,
	sat_id_10: i32,
	sat_id_11: i32,
	pdop: f64,
	hdop: f64,
	vdop: f64
}

impl Display for NmeaGpgsaS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPGSA Sentence:")?;
        writeln!(f, "  Mode: {}", self.mode as u8 as char)?;
        writeln!(f, "  Fix:  {}", self.fixtype)?;
        writeln!(f, "  PDOP: {:.2}", self.pdop)?;
        writeln!(f, "  HDOP: {:.2}", self.hdop)?;
        write!(f, "  VDOP: {:.2}", self.vdop)
    }
}