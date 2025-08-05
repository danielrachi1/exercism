const SECONDS_PER_EARTH_YEAR: f64 = 365.25 * 24.0 * 60.0 * 60.0; // 31,557,600.0

#[derive(Debug)]
pub struct Duration {seconds: u64}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {seconds: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! implement_planet_orbital_calculations {
    ( $( $planet_struct:ident : $orbital_period_expr:expr ),+ $(,)? ) => {
        $(
            impl Planet for $planet_struct {
                fn years_during(d: &Duration) -> f64 {
                    let duration_in_earth_years = d.seconds as f64 / SECONDS_PER_EARTH_YEAR;

                    let planet_orbital_period_in_earth_years: f64 = $orbital_period_expr;

                    duration_in_earth_years / planet_orbital_period_in_earth_years
                }
            }
        )+
    };
}

implement_planet_orbital_calculations! {
    Mercury: 0.2408467,
    Venus:   0.61519726,
    Earth:   1.0,
    Mars:    1.8808158,
    Jupiter: 11.862615,
    Saturn:  29.447498,
    Uranus:  84.016846,
    Neptune: 164.79132,
}