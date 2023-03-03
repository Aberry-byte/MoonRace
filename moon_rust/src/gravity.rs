/*
Author: Alex Huft
Description: Contain functions and constants relating to gravitational work
*/


const GRAVITATIONAL_CONSTANT: f64 = 6.67E-11;
const MASS_OF_EARTH_KG: f64 = 5.97219E24;
const MASS_OF_MOON_KG: f64 = 7.34767309E22;
const DISTANCE_TO_CENTER_OF_EARTH: f64 = 6.38E6;


pub fn gravitational_force(mass1: f64, mass2: f64, distance: f64) -> f64
{
    let top = GRAVITATIONAL_CONSTANT * mass1 * mass2;
    let bottom = distance.powi(2); // raise to the power of 2
    return top / bottom;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gravitation_force_test() {
        let result = gravitational_force(68.0,
                                         MASS_OF_EARTH_KG,
                                         DISTANCE_TO_CENTER_OF_EARTH);
        assert!((665.0..666.0).contains(&result));
    }

}
