mod gpgga;
mod gpgll;
mod gpgsa;
mod gpgsv;
mod gprmc;
mod gptxt;
mod gpvtg;

use std::{
    ffi::{c_char, CString}, fmt::{Alignment, Display}, io::{self, Error, ErrorKind}
};

const NMEA_CARDINAL_DIR_NORTH: char = 'N';
const NMEA_CARDINAL_DIR_EAST: char = 'E';
const NMEA_CARDINAL_DIR_SOUTH: char = 'S';
const NMEA_CARDINAL_DIR_WEST: char = 'W';
const NMEA_CARDINAL_DIR_UNKNOWN: char = '\0';

#[derive(Debug)]
pub struct Nmea {
    inner: *const NmeaS
}

impl Nmea {
    pub fn parse_nmea_string(nmea_sentence: &str) -> io::Result<Self> {
        unsafe {
            let nmea = nmea_parse(CString::new(nmea_sentence)?.as_ptr(), nmea_sentence.len(), 0);

            if nmea.is_null() {
                Err(Error::from(ErrorKind::InvalidInput))
            } else {
                Ok(Self { inner: nmea })
            }
        }
    }

    pub fn nmea_type(&self) -> NmeaT {
        unsafe {
            (&*self.inner).r#type
        }
    }

    pub fn gpgga(&self) -> io::Result<&gpgga::NmeaGpggaS> {
        if NmeaT::NmeaGPGGA != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gpgll(&self) -> io::Result<&gpgll::NmeaGpgllS> {
        if NmeaT::NmeaGPGLL != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gpgsa(&self) -> io::Result<&gpgsa::NmeaGpgsaS> {
        if NmeaT::NmeaGPGSA != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gpgsv(&self) -> io::Result<&gpgsv::NmeaGpgsvS> {
        if NmeaT::NmeaGPGSV != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gprmc(&self) -> io::Result<&gprmc::NmeaGprmcS> {
        if NmeaT::NmeaGPRMC != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gptxt(&self) -> io::Result<&gptxt::NmeaGptxtS> {
        if NmeaT::NmeaGPTXT != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }

    pub fn gpvtg(&self) -> io::Result<&gpvtg::NmeaGpvtgS> {
        if NmeaT::NmeaGPVTG != self.nmea_type() {
            return Err(Error::from(ErrorKind::InvalidData));
        } 
        
        unsafe {
            Ok(&*(self.inner as *const _))
        }
    }
}

impl Drop for Nmea {
    fn drop(&mut self) {
        unsafe {
            nmea_free(self.inner)
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NmeaT {
	NmeaUnknown,
	NmeaGPGGA,
	NmeaGPGLL,
	NmeaGPGSA,
	NmeaGPGSV,
	NmeaGPRMC,
	NmeaGPTXT,
	NmeaGPVTG
} 

#[repr(C)]
#[derive(Debug)]
struct NmeaS {
	r#type: NmeaT,
	errors: i32
}

// GPS position struct
#[repr(C)]
#[derive(Debug)]
struct NmeaPosition {
	minutes: f64,
	degrees: i32,
	cardinal: c_char 
}

impl Display for NmeaPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  Degrees: {}", self.degrees)?;
        writeln!(f, "  Minutes: {:.6}", self.minutes)?;
        write!(f, "  Cardinal: {}", self.cardinal as u8 as char)
    }
}

// seconds,  range 0 to 59
// minutes, range 0 to 59
// hours, range 0 to 23
// day of the month, range 1 to 31
// month, range 0 to 11
// The number of years since 1900
// day of the week, range 0 to 6
// day in the year, range 0 to 365
// daylight saving time
#[repr(C)]
#[derive(Debug)]
struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const c_char
}

impl Display for Tm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const M: &[&str] = &["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        
        write!(f, "{:02} {} {:02}:{:02}:{:02} {}", self.tm_mday, M[self.tm_mon as usize], self.tm_hour, self.tm_min, self.tm_sec, self.tm_year + 1900)
    }
}

extern {
    fn nmea_parse(sentence: *const c_char, length: usize, check_checksum: i32) -> *const NmeaS;

    fn nmea_free(data: *const NmeaS);
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::{BufRead, BufReader, Error as IoError, ErrorKind, Read}};

    use super::{Nmea, NmeaT};

    #[test]
    fn test_file() -> Result<(), Box<dyn Error>> {
        let file = "./test/parse_stdin_test_in.txt";
        let file_out = "./test/parse_stdin_test_out_expected.txt";
        let mut file = BufReader::new(File::open(file)?);
        let mut out = File::open(file_out)?;
        let mut expected = String::new();
        let mut storage = String::new();
        let mut setence = String::new();

        out.read_to_string(&mut expected);

        while let Ok(size) = file.read_line(&mut setence) {
            if size == 0 { break }
            
            let Ok(nmea) = Nmea::parse_nmea_string(&setence) else { setence.clear(); continue };
        
            if let Ok(value) = nmea_to_string(&nmea) {
                storage.push_str(&format!("{value}\n"));
            }
    
            setence.clear();
        }

        assert_eq!(storage, expected);
    
        Ok(())
    }
    
    fn nmea_to_string(nmea: &Nmea) -> Result<String, Box<dyn Error>> {
        match nmea.nmea_type() {
            NmeaT::NmeaGPGGA => Ok(nmea.gpgga()?.to_string()),
            NmeaT::NmeaGPGLL => Ok(nmea.gpgll()?.to_string()),
            NmeaT::NmeaGPGSA => Ok(nmea.gpgsa()?.to_string()),
            NmeaT::NmeaGPGSV => Ok(nmea.gpgsv()?.to_string()),
            NmeaT::NmeaGPRMC => Ok(nmea.gprmc()?.to_string()),
            NmeaT::NmeaGPTXT => Ok(nmea.gptxt()?.to_string()),
            NmeaT::NmeaGPVTG => Ok(nmea.gpvtg()?.to_string()),
            _ => Err(IoError::from(ErrorKind::InvalidData))?
        }
    }
}