//! Type-safe physical quantities backed by [`uom`](https://docs.rs/uom/0.38.0/uom/).
//!
//! A **quantity** is what you measure (`Length`, `Mass`, `Energy`, ...). A **unit**
//! tells us how to interpret a number (`meter`, `kilogram`, `joule`, ...).
//! All quantity types here use `f64` and SI base units internally. Choose units
//! at input/output boundaries; keep quantities typed while calculating.
//!
//! # Construct, combine, and convert
//!
//! ```
//! use tools::unit::{Length, length::{kilometer, meter}};
//!
//! let trip = Length::new::<kilometer>(1.0);
//! let detour = Length::new::<meter>(250.0);
//! let total = trip + detour;
//!
//! assert_eq!(total.get::<meter>(), 1_250.0);
//! assert_eq!(total.get::<kilometer>(), 1.25);
//! ```
//!
//! `new::<unit>(number)` attaches a unit; `get::<unit>()` extracts a number in
//! that unit. Conversion does not change the quantity's type.
//!
//! # Let arithmetic derive the result's dimension
//!
//! ```
//! use tools::unit::{
//!     Energy, Mass, Velocity, energy::joule, mass::kilogram,
//!     velocity::kilometer_per_hour,
//! };
//!
//! let mass = Mass::new::<kilogram>(2.0);
//! let speed = Velocity::new::<kilometer_per_hour>(36.0); // 10 m/s
//! let kinetic_energy: Energy = 0.5 * mass * speed * speed;
//!
//! assert!((kinetic_energy.get::<joule>() - 100.0).abs() < 1e-10);
//! ```
//!
//! Incompatible dimensions are compile errors, even if both use `f64`:
//!
//! ```compile_fail
//! use tools::unit::{Length, Time, length::meter, time::second};
//! let nonsense = Length::new::<meter>(1.0) + Time::new::<second>(2.0);
//! ```
//!
//! Units must also belong to the quantity being constructed:
//!
//! ```compile_fail
//! use tools::unit::{Mass, length::meter};
//! let mass = Mass::new::<meter>(5.0);
//! ```
//!
//! # Temperature: an absolute value differs from a change
//!
//! Celsius and Fahrenheit have offsets. Use [`ThermodynamicTemperature`] for
//! absolute temperatures and [`TemperatureInterval`] for changes:
//!
//! ```
//! use tools::unit::{
//!     TemperatureInterval, ThermodynamicTemperature,
//!     temperature_interval::kelvin, thermodynamic_temperature::degree_celsius,
//! };
//!
//! let room = ThermodynamicTemperature::new::<degree_celsius>(20.0);
//! let rise = TemperatureInterval::new::<kelvin>(5.0);
//! let warmer = room + rise;
//! assert!((warmer.get::<degree_celsius>() - 25.0).abs() < 1e-10);
//! ```
//!
//! Adding two absolute temperatures is deliberately unsupported:
//!
//! ```compile_fail
//! use tools::unit::{ThermodynamicTemperature, thermodynamic_temperature::kelvin};
//! let a = ThermodynamicTemperature::new::<kelvin>(280.0);
//! let b = ThermodynamicTemperature::new::<kelvin>(290.0);
//! let sum = a + b;
//! ```
//!
//! # Formatting and numeric validation
//!
//! ```
//! use tools::unit::{DisplayStyle, Length, QuantityExt, length::kilometer};
//!
//! let distance = Length::new::<kilometer>(1.5).ensure_finite()?;
//! let label = format!("{}", distance.into_format_args(kilometer, DisplayStyle::Abbreviation));
//! assert_eq!(label, "1.5 km");
//! # Ok::<(), tools::unit::NonFiniteQuantity>(())
//! ```
//!
//! Dimensional safety does **not** imply numeric or physical validity. These
//! types retain normal `f64` behavior: NaN, infinity, rounding, underflow, and
//! division by zero are possible. [`QuantityExt::ensure_finite`] checks the
//! stored SI value at an input or calculation boundary; it does not make later
//! arithmetic checked. A conversion with `get` can also overflow, so validate
//! its returned `f64` when needed. Domain rules such as positive mass or a
//! temperature above absolute zero belong in the calculator using the quantity.
//!
//! All `f64` quantity types are exported here. Common unit modules are exported
//! alongside them; the complete unit catalog is available through [`si`].

pub use uom::fmt::DisplayStyle;
pub use uom::si::{self, f64::*};
pub use uom::si::{
    acceleration, amount_of_substance, angle, area, electric_charge, electric_current,
    electric_potential, electrical_resistance, energy, force, frequency, length,
    luminous_intensity, mass, mass_density, molality, molar_concentration, molar_mass, momentum,
    power, pressure, ratio, temperature_interval, thermodynamic_temperature, time, velocity,
    volume,
};

/// A quantity's stored SI value was NaN or positive/negative infinity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("quantity must have a finite value in SI base units")]
pub struct NonFiniteQuantity;

/// Optional numeric validation shared by all `f64` quantities.
///
/// This leaves `uom`'s quantity types and arithmetic directly usable. Import
/// this trait to call `.ensure_finite()` where a calculator accepts input or
/// returns a result.
pub trait QuantityExt: Sized {
    /// Returns this quantity unchanged if its stored SI value is finite.
    ///
    /// Zero, negative values, and subnormal values are accepted. This checks
    /// numeric finiteness only, not whether the value makes physical sense.
    ///
    /// # Errors
    ///
    /// Returns [`NonFiniteQuantity`] for NaN or either infinity, including
    /// overflow produced by conversion to SI units or earlier arithmetic.
    fn ensure_finite(self) -> Result<Self, NonFiniteQuantity>;
}

impl<D, U> QuantityExt for si::Quantity<D, U, f64>
where
    D: si::Dimension + ?Sized,
    U: si::Units<f64> + ?Sized,
{
    fn ensure_finite(self) -> Result<Self, NonFiniteQuantity> {
        if self.is_finite() {
            Ok(self)
        } else {
            Err(NonFiniteQuantity)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit conversions use floating point factors; compare with a tolerance.
    fn assert_close(actual: f64, expected: f64) {
        let tolerance = 1e-10 * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn mixed_units_add_and_convert() {
        let distance = Length::new::<length::kilometer>(1.0) + Length::new::<length::meter>(250.0);

        assert_close(distance.get::<length::meter>(), 1_250.0);
        assert_close(distance.get::<length::kilometer>(), 1.25);
        assert_close(
            Length::new::<length::foot>(1.0).get::<length::meter>(),
            0.3048,
        );
    }

    #[test]
    fn arithmetic_derives_velocity_and_kinetic_energy() {
        let distance = Length::new::<length::meter>(100.0);
        let duration = Time::new::<time::second>(10.0);
        let speed: Velocity = distance / duration;
        let mass = Mass::new::<mass::gram>(2_000.0);
        let energy: Energy = 0.5 * mass * speed * speed;

        assert_close(speed.get::<velocity::kilometer_per_hour>(), 36.0);
        assert_close(energy.get::<energy::joule>(), 100.0);
    }

    #[test]
    fn derived_geometry_converts_to_liters() {
        let side = Length::new::<length::centimeter>(10.0);
        let area: Area = side * side;
        let volume: Volume = area * side;

        assert_close(area.get::<area::square_meter>(), 0.01);
        assert_close(volume.get::<volume::liter>(), 1.0);
    }

    #[test]
    fn electrical_units_are_available() {
        let voltage = ElectricPotential::new::<electric_potential::volt>(12.0);
        let current = ElectricCurrent::new::<electric_current::ampere>(2.0);
        let power: Power = voltage * current;

        assert_close(power.get::<power::watt>(), 24.0);
    }

    #[test]
    fn absolute_temperatures_convert_with_offsets() {
        let freezing =
            ThermodynamicTemperature::new::<thermodynamic_temperature::degree_celsius>(0.0);

        assert_close(freezing.get::<thermodynamic_temperature::kelvin>(), 273.15);
        assert_close(
            freezing.get::<thermodynamic_temperature::degree_fahrenheit>(),
            32.0,
        );
        let boiling =
            ThermodynamicTemperature::new::<thermodynamic_temperature::degree_fahrenheit>(212.0);
        assert_close(
            boiling.get::<thermodynamic_temperature::degree_celsius>(),
            100.0,
        );
    }

    #[test]
    fn temperature_intervals_do_not_use_absolute_offsets() {
        let room = ThermodynamicTemperature::new::<thermodynamic_temperature::degree_celsius>(20.0);
        let rise = TemperatureInterval::new::<temperature_interval::degree_fahrenheit>(9.0);

        assert_close(rise.get::<temperature_interval::kelvin>(), 5.0);
        assert_close(
            (room + rise).get::<thermodynamic_temperature::degree_celsius>(),
            25.0,
        );
    }

    #[test]
    fn finite_validation_preserves_values_including_subnormals() {
        for value in [0.0, -0.0, -5.0, 5.0, f64::from_bits(1), f64::MAX] {
            let quantity = Length::new::<length::meter>(value).ensure_finite().unwrap();
            assert_eq!(quantity.get::<length::meter>().to_bits(), value.to_bits());
        }
    }

    #[test]
    fn finite_validation_rejects_invalid_input_and_conversion_overflow() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                Length::new::<length::meter>(value).ensure_finite(),
                Err(NonFiniteQuantity)
            );
        }
        assert_eq!(
            Length::new::<length::kilometer>(f64::MAX).ensure_finite(),
            Err(NonFiniteQuantity)
        );
    }

    #[test]
    fn finite_validation_catches_invalid_calculation_results() {
        let huge = Length::new::<length::meter>(f64::MAX);
        assert_eq!((huge + huge).ensure_finite(), Err(NonFiniteQuantity));

        let distance = Length::new::<length::meter>(1.0);
        let zero_time = Time::new::<time::second>(0.0);
        assert_eq!(
            (distance / zero_time).ensure_finite(),
            Err(NonFiniteQuantity)
        );
        assert_eq!(
            (distance * f64::NAN).ensure_finite(),
            Err(NonFiniteQuantity)
        );
    }
}
