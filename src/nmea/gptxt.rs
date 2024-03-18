use std::{ffi::{c_char, CString}, fmt::Display, mem::ManuallyDrop};

use super::NmeaS;

const NMEA_GPTXT_TEXT_SIZE: usize = 64;

#[repr(C)]
#[derive(Debug)]
pub struct NmeaGptxtS {
	base: NmeaS,
	id_00: i32,
	id_01: i32,
	id_02: i32,
	text: [c_char; NMEA_GPTXT_TEXT_SIZE]
}

impl Display for NmeaGptxtS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPTXT Sentence:")?;
        writeln!(f, "  ID: {} {} {}", self.id_00, self.id_01, self.id_02)?;
        write!(f, "  {}", unsafe { ManuallyDrop::new(CString::from_raw(self.text.as_ptr() as *mut i8)).to_string_lossy() })
    }
}