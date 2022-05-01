// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEARS_PER_SECOND: f64 = 1.0 / 31557600.0;
const MERCURY_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 0.2408467;
const VENUS_YEARSS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 0.61519726;
const MARS_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 1.8808158;
const JUPITER_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 11.862615;
const SATURN_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 29.447498;
const URANUS_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 84.016846;
const NEPTUNE_YEARS_PER_SECOND: f64 = EARTH_YEARS_PER_SECOND / 164.79132;

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
    ($planet:ident, $year_per_second:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 * $year_per_second
            }
        }
    };
}

impl_planet!(Earth, EARTH_YEARS_PER_SECOND);
impl_planet!(Mercury, MERCURY_YEARS_PER_SECOND);
impl_planet!(Venus, VENUS_YEARSS_PER_SECOND);
impl_planet!(Mars, MARS_YEARS_PER_SECOND);
impl_planet!(Jupiter, JUPITER_YEARS_PER_SECOND);
impl_planet!(Saturn, SATURN_YEARS_PER_SECOND);
impl_planet!(Uranus, URANUS_YEARS_PER_SECOND);
impl_planet!(Neptune, NEPTUNE_YEARS_PER_SECOND);