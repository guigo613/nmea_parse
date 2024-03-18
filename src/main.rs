#![allow(unused)]

mod nmea;

use std::{error::Error, io::stdin};

use nmea::{Nmea, NmeaT};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = stdin();

    let mut setence = String::new();
    while let Ok(size) = stdin.read_line(&mut setence) {
        if size == 0 { break }
        
        let Ok(nmea) = Nmea::parse_nmea_string(&setence) else { setence.clear(); continue };
    
        print_nmea(&nmea)?;

        setence.clear();
    }

    Ok(())
}

fn print_nmea(nmea: &Nmea) -> Result<(), Box<dyn Error>> {
    match nmea.nmea_type() {
        NmeaT::NmeaGPGGA => println!("{}", nmea.gpgga()?),
        NmeaT::NmeaGPGLL => println!("{}", nmea.gpgll()?),
        NmeaT::NmeaGPGSA => println!("{}", nmea.gpgsa()?),
        NmeaT::NmeaGPGSV => println!("{}", nmea.gpgsv()?),
        NmeaT::NmeaGPRMC => println!("{}", nmea.gprmc()?),
        NmeaT::NmeaGPTXT => println!("{}", nmea.gptxt()?),
        NmeaT::NmeaGPVTG => println!("{}", nmea.gpvtg()?),
        _ => ()
    }

    Ok(())
}

