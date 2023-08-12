//! This module contains anything related to interpolation.


#[derive( Copy, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
/// The `Interpolation` Type.
/// This represents the various forms of interpolation that can be performed.
pub enum Interpolation {
    /// `0`
    Step = 0,
    /// `t`
    Linear = 1,
    /// `t * t * (3 - 2 * t)`
    Smooth = 2,
    /// `t.powi(2)`
    Ramp = 3,
}

impl From<u8> for Interpolation {
    fn from(raw: u8) -> Interpolation {
        match raw {
            0 => Interpolation::Step,
            1 => Interpolation::Linear,
            2 => Interpolation::Smooth,
            3 => Interpolation::Ramp,
            _ => Interpolation::Step,
        }
    }
}

impl Interpolation {
    /// This performs the interpolation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_rocket::interpolation::Interpolation;
    /// assert_eq!(Interpolation::Linear.interpolate(0.5), 0.5);
    /// ```
    ///
    /// ```
    /// # use rust_rocket::interpolation::Interpolation;
    /// assert_eq!(Interpolation::Step.interpolate(0.5), 0.);
    /// ```
    pub fn interpolate(&self, t: f32) -> f32 {
        // println!("interp {t} {self:?}");
        match *self {
            Interpolation::Step => 0.0,
            Interpolation::Linear => t,
            Interpolation::Smooth => t * t * (3.0 - 2.0 * t),
            Interpolation::Ramp => t.powi(2),
        }
    }
}
