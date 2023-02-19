#![no_std]

//! Calculation methods for platinum type RTD temperature sensors.
//! All temperature related calculations are based on DIN EN 60751:2009-05.
//! The polynomials for PT100 and PT1000 for temperature calculation at below 0°C are from
//! https://github.com/ulikoehler/UliEngineering/blob/master/UliEngineering/Physics/RTD.py. See also
//! https://techoverflow.net/2016/01/02/accurate-calculation-of-pt100pt1000-temperature-from-resistance/.

use libm::{
    powf,
    sqrtf,
    floorf,
};

#[allow(dead_code)]
#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum ADCRes {
    B8 = 255,
    B10 = 1_023,
    B12 = 4_095,
    B14 = 16_383,
    B16 = 65_535,
    B18 = 262_143,
    B20 = 1_048_575,
    B22 = 4_194_303,
    B24 = 16_777_215,
}

#[allow(dead_code)]
#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum RTDType {
    PT100 = 100,
    PT200 = 200,
    PT500 = 500,
    PT1000 = 1000,
}

#[allow(dead_code)]
#[non_exhaustive]
struct RTDCorrection;

impl RTDCorrection {
    pub const PT100: Polynomial = [1.51892983e-10, -2.85842067e-08, -5.34227299e-06,
    1.80282972e-03, -1.61875985e-01, 4.84112370e+00];
    pub const PT200: Polynomial = [0_f32; 6]; // FIXME: Precalculate correctional polynomial for PT200
    pub const PT500: Polynomial = [0_f32; 6]; // FIXME: Precalculate correctional polynomial for PT500
    pub const PT1000: Polynomial = [1.51892983e-15, -2.85842067e-12, -5.34227299e-09,
    1.80282972e-05, -1.61875985e-02, 4.84112370e+00];
}
type Polynomial = [f32; 6];

const A: f32 = 3.9083e-3;
const B: f32 = -5.7750e-7;
const C: f32 = -4.1830e-12;

/// Calculate temperature of RTD from resistance value.
/// Allowed temperature range: -200–850°C.
#[allow(dead_code)]
pub fn calc_t(r: f32, r_0: RTDType) -> Result<f32, Error> {
    let r_min = floorf(calc_r(-200_f32, r_0).unwrap()) as i32;
    let r_max = floorf(calc_r(850_f32, r_0).unwrap()) as i32;

    // set correctional polynomial for t < 0°C
    let corr_poly: Result<[f32; 6], Error> = match r_0 {
        RTDType::PT100 => Ok(RTDCorrection::PT100),
        RTDType::PT200 => Ok(RTDCorrection::PT200),
        RTDType::PT500 => Ok(RTDCorrection::PT500),
        RTDType::PT1000 => Ok(RTDCorrection::PT1000),
    };

    // cast r_0 to f32 for calculation
    let r_0 = r_0 as i32 as f32;
    let mut t = ( -r_0 * A + sqrtf( powf(r_0, 2_f32) * powf(A, 2_f32) - 4_f32 * r_0 * B * ( r_0 - r as f32 ) ) ) / ( 2_f32 * r_0 as f32 * B );

    match corr_poly {
        Ok(poly) => {
            match (floorf(r) as i32, r_0 as i32) {
                (r, r_0) if r_0 <= r && r <= r_max => {
                    // t >= 0°C
                    Ok(t)
                },
                (r, r_0) if r_min <= r && r < r_0 => {
                    // t < 0°C
                    // Apply the correctional polynomial
                    t += poly_correction(r as f32, poly);
                    Ok(t)
                },
                _ => Err(Error::OutOfBounds),
            }
        },
        Err(_) => Err(Error::NonexistentType),
    }
}

/// Calculate resistance of RTD for a specified temperature.
/// Allowed temperature range: -200–850°C.
/// For temperatures below 0°C a small error (58.6uK max. over the full range) is introduced due to the use of polynomial approximation.
#[allow(dead_code)]
pub fn calc_r(t: f32, r_0: RTDType) -> Result<f32, Error> {
    let r_0 = r_0 as i32;
    match floorf(t) as i32 {
        0..=850 => Ok(r_0 as f32 * ( 1_f32 + A * t + B * powf(t, 2_f32) )),
        -200..=-1 => Ok(r_0 as f32 * ( 1_f32 + A * t + B * powf(t, 2_f32) + C * ( t - 100_f32 ) * powf(t, 3_f32) )),
        _ => Err(Error::OutOfBounds),
    }
}

/// Convert digital value of relative measurement for n bit ADC to resistance.
#[allow(dead_code)]
pub fn conv_d_val_to_r(d_val: u32, r_ref: u32, res: ADCRes, pga_gain: u32) -> Result<f32, Error> {
    let res = res as u32;
    match d_val {
        d if d <= res => Ok(d_val as f32 * r_ref as f32 / ( res as f32 * pga_gain as f32)),
        _ => Err(Error::OutOfBounds),
    }
}

/// Calculate polynomial correctional factor for t < 0°C.
#[allow(dead_code)]
fn poly_correction(r: f32, poly: Polynomial) -> f32 {
    let mut res = 0_f32;
    for (i, factor) in poly.iter().enumerate() {
        res += factor * powf(r, i as f32);
    };    
    res
}

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    NonexistentType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistance_calculation() {
        let t = 0.0;
        
        let r = calc_r(t, RTDType::PT100).unwrap();
        assert_eq!(r, 100_f32);
    }

    #[test]
    fn temperature_calculation() {
        let r = 100.0;

        let t = calc_t(r, RTDType::PT100).unwrap();
        assert_eq!(t, 0_f32);
    }
}