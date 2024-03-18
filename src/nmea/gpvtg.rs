use std::fmt::Display;

use super::NmeaS;

#[repr(C)]
#[derive(Debug)]
pub struct NmeaGpvtgS {
	base: NmeaS,
	track_deg: f64,
	gndspd_knots: f64,
	gndspd_kmph: f64
}

impl Display for NmeaGpvtgS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GPVTG Sentence:")?;
        writeln!(f, "  Track [deg]:   {:.2}", self.track_deg)?;
        writeln!(f, "  Speed [kmph]:  {:.2}", self.gndspd_kmph)?;
        write!(f, "  Speed [knots]: {:.2}", self.gndspd_knots)
    }
}